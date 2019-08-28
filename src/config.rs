/// Describes the mode of the application (build|run).
#[derive(Debug, PartialEq)]
pub enum Mode {
    /// Compiles the current package.
    Build,
    /// Compiles and run the current package.
    Run,
    // todo: Deploy
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Build
    }
}

impl From<&str> for Mode {
    fn from(s: &str) -> Self {
        match s {
            "build" => Mode::Build,
            "run" => Mode::Run,
            _ => {
                panic!("Unknown mode: {}", s);
            }
        }
    }
}

impl From<String> for Mode {
    fn from(s: String) -> Self {
        Mode::from(s.as_str())
    }
}

/// Describes the target platform (electron|browser|android|ios).
#[derive(Debug, PartialEq)]
pub enum Target {
    Electron,
    Browser,
    Android,
    IOS,
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
            "ios" => Target::IOS,
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
    pub mode: Mode,
    pub target: Target,
    pub package: Package,
}

impl From<Vec<String>> for Config {
    fn from(args: Vec<String>) -> Self {
        let mut mode = None;
        let mut target = None;
        let mut package = None;
        let mut found_target = false;
        let mut package_arg = String::default();

        for arg in args {
            // mode must be the first argument
            if mode.is_none() {
                mode = Some(Mode::from(arg));
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
                _ => {}
            }

            if found_target && target.is_none() {
                target = Some(Target::from(arg.clone()));
            }

            if !package_arg.is_empty() && package.is_none() {
                package = Some(Package::from((package_arg.clone(), arg.clone())));
            }
        }

        if mode.is_none() {
            panic!("No mode (build|run) is given.")
        }

        if target.is_none() {
            panic!("No target (electron|browser|android is given.");
        }

        if package.is_none() {
            package = Some(Package::default());
        }

        // unwrap because if not set the application panics before.
        Config {
            mode: mode.unwrap(),
            target: target.unwrap(),
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
        assert_eq!(config.mode, Mode::Build);
        assert_eq!(config.target, Target::Electron);
        assert_eq!(config.package, Package::Bin("test".to_string()));

        let args: Vec<String> = vec!["build", "--target", "browser", "--bin", "test"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        let config = Config::from(args);
        assert_eq!(config.mode, Mode::Build);
        assert_eq!(config.target, Target::Browser);
        assert_eq!(config.package, Package::Bin("test".to_string()));

        let args: Vec<String> = vec!["build", "--target", "android", "--bin", "test"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        let config = Config::from(args);
        assert_eq!(config.mode, Mode::Build);
        assert_eq!(config.target, Target::Android);
        assert_eq!(config.package, Package::Bin("test".to_string()));

        let args: Vec<String> = vec!["build", "--target", "ios", "--bin", "test"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        let config = Config::from(args);
        assert_eq!(config.mode, Mode::Build);
        assert_eq!(config.target, Target::IOS);
        assert_eq!(config.package, Package::Bin("test".to_string()));

        let args: Vec<String> = vec!["build", "--target", "electron", "--example", "test"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        let config = Config::from(args);
        assert_eq!(config.mode, Mode::Build);
        assert_eq!(config.target, Target::Electron);
        assert_eq!(config.package, Package::Example("test".to_string()));

        let args: Vec<String> = vec!["build", "--target", "electron"]
            .iter()
            .map(|a| a.to_string())
            .collect();

        let config = Config::from(args);
        assert_eq!(config.mode, Mode::Build);
        assert_eq!(config.target, Target::Electron);
        assert_eq!(config.package, Package::Scope);

        let args: Vec<String> = vec!["run", "--target", "electron"]
            .iter()
            .map(|a| a.to_string())
            .collect();

        let config = Config::from(args);
        assert_eq!(config.mode, Mode::Run);
        assert_eq!(config.target, Target::Electron);
        assert_eq!(config.package, Package::Scope);
    }
}