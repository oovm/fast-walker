use super::*;
use std::task::Poll;

pub struct WalkSearcher {
    result_queue: WalkResultQueue,
}

impl Stream for WalkSearcher {}

impl<'i> IntoStream for &'i WalkPlan {
    type Item = WalkItem;
    type Error = std::io::Error;
    type Stream = WalkSearcher;

    fn into_stream(self) -> Self::Stream {
        let result = WalkResultQueue::new();
        let result_queue = result.clone();
        let tasks = WalkTaskQueue::new(self.depth_first);
        tasks.send_roots(&self.check_list);
        let reject_directory = self.reject_when;
        // let finish_condition = self.finish_when;
        let handler = tokio::spawn(async move {
            while let Some((path, depth)) = tasks.receive().await {
                if reject_directory(&path, depth) {
                    continue;
                }
                match std::fs::read_dir(&path) {
                    Ok(read_dir) => {
                        for item in read_dir {
                            match item {
                                Ok(dir_entry) => match dir_entry.file_type() {
                                    Ok(file_type) => {
                                        let path = dir_entry.path();
                                        match file_type.is_dir() {
                                            true => {
                                                tasks.send(&path, depth + 1);
                                                result.send_directory(path)
                                            }
                                            false => {
                                                result.send_file(path);
                                            }
                                        }
                                    }
                                    Err(e) => result.send_error(path.clone(), e),
                                },
                                Err(e) => result.send_error(path.clone(), e),
                            }
                        }
                    }
                    Err(e) => result.send_error(path, e),
                }
            }
        });
        WalkSearcher { result_queue }
    }
}
