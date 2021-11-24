//! Types & Definitions needed to keep compatibility with existing API

use super::*;

impl<'data> SymCache<'data> {
    /// Returns true if line information is included.
    pub fn has_line_info(&self) -> bool {
        self.has_file_info() && self.source_locations.iter().any(|sl| sl.line > 0)
    }

    /// Returns true if file information is included.
    pub fn has_file_info(&self) -> bool {
        !self.files.is_empty()
    }

    /// An iterator over the functions in this SymCache.
    pub fn functions(&self) -> FunctionIter<'data> {
        FunctionIter {
            cache: self.clone(),
            function_idx: 0,
        }
    }

    /// An iterator over the files in this SymCache.
    pub fn files(&self) -> FileIter<'data, '_> {
        FileIter {
            cache: self,
            file_idx: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileIter<'data, 'cache> {
    cache: &'cache SymCache<'data>,
    file_idx: u32,
}

impl<'data, 'cache> Iterator for FileIter<'data, 'cache> {
    type Item = File<'data, 'cache>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cache
            .files
            .get(self.file_idx as usize)
            .map(|raw_file| {
                self.file_idx += 1;
                File {
                    cache: self.cache,
                    file: raw_file,
                }
            })
    }
}

#[derive(Debug, Clone)]
pub struct FunctionIter<'data> {
    cache: SymCache<'data>,
    function_idx: u32,
}

impl<'data> Iterator for FunctionIter<'data> {
    type Item = Function<'data>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cache
            .functions
            .get(self.function_idx as usize)
            .map(|raw_function| {
                self.function_idx += 1;
                Function {
                    cache: &self.cache,
                    function: raw_function,
                }
            })
    }
}
