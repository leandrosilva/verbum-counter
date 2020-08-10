use multimap::MultiMap;
use neon::prelude::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, Read};

struct WordCount {
    word: String,
    count: f64,
}

type WordCountResult = Vec<WordCount>;

struct WordCountTask {
    filepath: String,
}

impl Task for WordCountTask {
    type Output = WordCountResult;
    type Error = String;
    type JsEvent = JsObject;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        match count_words_sync(&self.filepath) {
            Ok(result) => Ok(result),
            Err(err) => Err(err.to_string()),
        }
    }

    fn complete(
        self,
        mut cx: TaskContext,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<JsObject> {
        if result.is_err() {
            let err = result.err().unwrap();
            return cx.throw_error(err);
        }

        let wc_result = result.unwrap();

        let js_array = JsArray::new(&mut cx, wc_result.len() as u32);
        wc_result.iter().enumerate().for_each(|e| {
            let (i, wc) = e;
            let js_string = cx.string(&wc.word);
            let js_number = cx.number(wc.count);
            let js_object = JsObject::new(&mut cx);
            js_object.set(&mut cx, "word", js_string).unwrap();
            js_object.set(&mut cx, "count", js_number).unwrap();
            let _ = js_array.set(&mut cx, i as u32, js_object);
        });
        let js_object = JsObject::new(&mut cx);
        js_object.set(&mut cx, "data", js_array).unwrap();

        println!(">> Ready to complete...");

        Ok(js_object)
    }
}

// Actual word counting
//

fn count_words_sync(filepath: &String) -> Result<WordCountResult, io::Error> {
    println!(">> Will start now...");

    let file_content = read_file_content(&filepath)?;
    let words = file_content.split_whitespace().collect::<Vec<_>>();

    // Step #1: map
    let mapped: Vec<_> = words
        .into_par_iter()
        .map(|word| (word.to_string(), ()))
        .collect();

    // Step #2: group by word
    let grouped = mapped
        .into_iter()
        .collect::<MultiMap<_, _>>()
        .into_iter()
        .collect::<Vec<_>>();

    // Step #3: reduce by word
    let mut reduced: Vec<_> = grouped
        .into_par_iter()
        .map(|kv| (kv.0, kv.1.len()))
        .collect();

    // Step #4: order by most frequent
    reduced.sort_by(|a, b| b.1.cmp(&a.1));

    // Step #5: final result
    let result: WordCountResult = reduced
        .into_par_iter()
        .map(|kv| WordCount {
            word: kv.0,
            count: kv.1 as f64,
        })
        .collect();

    Ok(result)
}

// Helper functions
//

fn read_file_content(filepath: &String) -> io::Result<String> {
    let mut filedata = String::new();
    let mut file = File::open(&filepath)?;
    file.read_to_string(&mut filedata)?;
    Ok(filedata)
}

// Exported function
//

fn count_words(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let filepath = cx.argument::<JsString>(0)?.value();
    let callback = cx.argument::<JsFunction>(1)?;

    let task = WordCountTask { filepath };
    println!(">> Will schedule...");
    task.schedule(callback);
    println!(">> It is scheduled...");

    Ok(cx.undefined())
}

register_module!(mut cx, { cx.export_function("count_words", count_words) });
