#[derive(Clone, Debug)]

pub enum LogLevel {
    Profiling,
    Info,
    Verbose,
}

impl LogLevel {
    pub fn is_profiling(&self) -> bool {
        return match self {
            LogLevel::Profiling => true,
            _ => false,
        }
    }

    // pub fn is_info(&self) -> bool {
    //     return match self {
    //         LogLevel::Info => true,
    //         _ => false,
    //     }
    // }

    pub fn is_verbose(&self) -> bool {
        return match self {
            LogLevel::Verbose => true,
            _ => false,
        }
    }
}