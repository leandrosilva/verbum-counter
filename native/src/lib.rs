use neon::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

struct WordCount {
    word: String,
    count: u64,
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
            let js_number = cx.number(wc.count as f64);
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

    let mut counted: HashMap<&str, u64> = HashMap::new();
    for word in words {
        let counter = counted.entry(word).or_insert(0);
        *counter += 1;
    }

    let mut sorted: Vec<_> = counted
        .into_par_iter()
        .map(|kv| WordCount {
            word: kv.0.to_string(),
            count: kv.1,
        })
        .collect();
    sorted.par_sort_unstable_by(|a, b| b.count.cmp(&a.count));

    Ok(sorted)
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
