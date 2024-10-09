use std::ops::Range;

pub struct SampleSpan {
    position: usize,
    length: usize,
}

impl SampleSpan {
    pub fn new(position: usize, length: usize) -> Self {
        Self {
            position,
            length
        }
    }
    
    pub fn range(&self) -> Range<usize> {
        Range {
            start: self.start(),
            end: self.end()
        }
    }

    pub fn start(&self) -> usize {
        self.position
    }

    pub fn end(&self) -> usize {
        self.position + self.length
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn pull_back(&self, back: usize) -> Self {
        Self {
            position: self.position - back,
            length: self.length
        }
    }
    
    pub fn truncate(&self, length: usize) -> Self {
        Self {
            length,
            ..*self
        }
    }
}

impl From<Range<usize>> for SampleSpan {
    fn from(value: Range<usize>) -> Self {
        Self {
            position: value.start,
            length: value.end - value.start
        }
    }
}