/// Describes the task task of the application (debug release).
#[derive(Debug, PartialEq)]
pub enum Mode {
    /// Debug task.
    Debug,
    /// Release task.
    Release,
}

impl From<&str> for Mode {
    fn from(s: &str) -> Self {
        match s {
            "debug" => Mode::Debug,
            "release" => Mode::Release,
            _ => {
                panic!("Unknown  mode: {}", s);
            }
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Debug
    }
}

/// Describes the task of the application (build|run|clear|deploy).
#[derive(Debug, PartialEq)]
pub enum Task {
    /// Compiles the current package.
    Build,
    /// Compiles and run the current package.
    Run,
    /// Remove all files.
    Clear,
    /// Deploy the project.
    Deploy,
}

impl Default for Task {
    fn default() -> Self {
        Task::Build
    }
}

impl From<&str> for Task {
    fn from(s: &str) -> Self {
        match s {
            "build" => Task::Build,
            "run" => Task::Run,
            "clear" => Task::Clear,
            "deploy" => Task::Deploy,
            _ => {
                panic!("Unknown task: {}", s);
            }
        }
    }
}

impl From<String> for Task {
    fn from(s: String) -> Self {
        Task::from(s.as_str())
    }
}

/// Describes the target platform (electron|browser|android).
#[derive(Debug, PartialEq)]
pub enum Target {
    Electron,
    Browser,
    Android,
    // IOS,
    /// Used only by clear task.
    None,
}

impl Default for Target {
    fn default() -> Self {
        Target::Electron
    }
}

impl From<&str> for Target {
    fn from(s: &str) -> Self {
        match s {
            "electron" => Target::Electron,
            "browser" => Target::Browser,
            "android" => Target::Android,
            // "ios" => Target::IOS,
            _ => {
                panic!("Unknown target: {}", s);
            }
        }
    }
}

impl From<String> for Target {
    fn from(s: String) -> Self {
        Target::from(s.as_str())
    }
}

/// Describes the package to compile and or run.
#[derive(Debug, PartialEq)]
pub enum Package {
    /// Use the given bin package.
    Bin(String),
    /// Use the given example package.
    Example(String),
    /// Use the scope package.
    Scope,
    /// Used only by clear task.
    None,
}

impl Default for Package {
    fn default() -> Self {
        Package::Scope
    }
}

impl From<(&str, &str)> for Package {
    fn from(s: (&str, &str)) -> Self {
        match s.0 {
            "--bin" => Package::Bin(s.1.to_string()),
            "--example" => Package::Example(s.1.to_string()),
            _ => {
                panic!("Unknown build flag: {}", s.0);
            }
        }
    }
}

impl From<(String, String)> for Package {
    fn from(s: (String, String)) -> Self {
        Package::from((s.0.as_str(), s.1.as_str()))
    }
}

#[derive(Debug)]
pub struct Config {
    pub task: Task,
    pub mode: Mode,
    pub target: Target,
    pub package: Package,
}

impl From<Vec<String>> for Config {
    fn from(args: Vec<String>) -> Self {
        let mut task = None;
        let mut mode = Mode::default();
        let mut target = Target::Electron;
        let mut package = None;
        let mut found_target = false;
        let mut package_arg = String::default();

        for arg in args {
            // task must be the first argument
            if task.is_none() {
                task = Some(Task::from(arg));
                if *task.as_ref().unwrap() == Task::Clear {
                    target = Target::None;
                    package = Some(Package::None);
                    break;
                }
                continue;
            }

            match arg.as_str() {
                "--target" => {
                    found_target = true;
                    continue;
                }
                "--bin" => {
                    package_arg = arg.clone();
                    continue;
                }
                "--example" => {
                    package_arg = arg.clone();
                    continue;
                }
                "--release" => {
                    mode = Mode::Release;
                    continue;
                }
                _ => {}
            }

            if found_target {
                target = Target::from(arg.clone());
                found_target = false;
            }

            if !package_arg.is_empty() && package.is_none() {
                package = Some(Package::from((package_arg.clone(), arg.clone())));
            }
        }

        if task.is_none() {
            panic!("No task (build|run|clear) is given.")
        }

        if package.is_none() {
            package = Some(Package::default());
        }

        // unwrap because if not set the application panics before.
        Config {
            task: task.unwrap(),
            mode,
            target: target,
            package: package.unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let args: Vec<String> = vec!["build", "--target", "electron", "--bin", "test"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        let config = Config::from(args);
        assert_eq!(config.task, Task::Build);
        assert_eq!(config.target, Target::Electron);
        assert_eq!(config.package, Package::Bin("test".to_string()));

        let args: Vec<String> = vec!["build", "--target", "browser", "--bin", "test"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        let config = Config::from(args);
        assert_eq!(config.task, Task::Build);
        assert_eq!(config.target, Target::Browser);
        assert_eq!(config.package, Package::Bin("test".to_string()));

        let args: Vec<String> = vec!["build", "--target", "android", "--bin", "test"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        let config = Config::from(args);
        assert_eq!(config.task, Task::Build);
        assert_eq!(config.target, Target::Android);
        assert_eq!(config.package, Package::Bin("test".to_string()));

        let args: Vec<String> = vec!["build", "--target", "electron", "--bin", "test"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        let config = Config::from(args);
        assert_eq!(config.task, Task::Build);
        assert_eq!(config.target, Target::Electron);
        assert_eq!(config.package, Package::Bin("test".to_string()));

        let args: Vec<String> = vec!["build", "--target", "electron", "--example", "test"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        let config = Config::from(args);
        assert_eq!(config.task, Task::Build);
        assert_eq!(config.target, Target::Electron);
        assert_eq!(config.package, Package::Example("test".to_string()));

        let args: Vec<String> = vec!["build", "--target", "electron"]
            .iter()
            .map(|a| a.to_string())
            .collect();

        let config = Config::from(args);
        assert_eq!(config.task, Task::Build);
        assert_eq!(config.target, Target::Electron);
        assert_eq!(config.package, Package::Scope);

        let args: Vec<String> = vec!["run", "--target", "electron"]
            .iter()
            .map(|a| a.to_string())
            .collect();

        let config = Config::from(args);
        assert_eq!(config.task, Task::Run);
        assert_eq!(config.target, Target::Electron);
        assert_eq!(config.package, Package::Scope);

        let args: Vec<String> = vec!["clear"].iter().map(|a| a.to_string()).collect();
        let config = Config::from(args);
        assert_eq!(config.task, Task::Clear);
        assert_eq!(config.target, Target::None);
        assert_eq!(config.package, Package::None);

        let args: Vec<String> = vec!["deploy", "--target", "electron"]
            .iter()
            .map(|a| a.to_string())
            .collect();

        let config = Config::from(args);
        assert_eq!(config.task, Task::Deploy);
        assert_eq!(config.target, Target::Electron);
        assert_eq!(config.package, Package::Scope);
    }
}
