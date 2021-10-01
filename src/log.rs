use colored::Colorize;

pub enum LogType {
    LogInfo,
    LogWarn,
    LogErr,
    LogCrit
}

/// logs data with color and handy pre-labeling
pub fn log(ltype: LogType, string: String) {
    let strng = &string[..];
    _log(ltype, strng);
}

pub fn _log(ltype: LogType, string: &str) {
    // print fancy colors depending the input type
    match ltype {
        LogType::LogInfo    => println!("{} {}", "[INFO]:".green(), string.green()),
        LogType::LogWarn    => println!("{} {}", "[WARN]:".yellow(), string.yellow()),
        LogType::LogErr     => println!("{} {}", "[ERR]:".red(), string.red()),
        LogType::LogCrit    => println!("{} {}", "[CRIT]:".bold().red(), string.bold().red())
    }
}