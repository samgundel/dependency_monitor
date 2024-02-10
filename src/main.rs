use std::collections::HashMap;


#[derive(Debug)]
struct DependencyMonitor {
    pull_requests: HashMap<u32, PullRequest>
}

impl DependencyMonitor {
    fn new() -> DependencyMonitor {
        DependencyMonitor {
            pull_requests: HashMap::new()
        }
    }
}

#[derive(Debug)]
struct Monitor<'a> {
    dependency_monitor: &'a mut DependencyMonitor
}

impl Monitor<'a> {
    fn new<'a>(dependency_monitor: &'a mut DependencyMonitor) -> Monitor {
        Monitor {
            dependency_monitor
        }
    }
}

#[derive(Debug)]
struct PullRequest {
   build_status: bool,
   dependency_status: bool,
   number: u32,
   dependencies: Vec<u32>
}

impl PullRequest {
    fn new(number:u32) -> PullRequest {
        PullRequest {
            build_status: false,
            dependency_status: false,
            number,
            dependencies: Vec::new() 
        }
    }
    fn set_build_status(&mut self, build_status: bool) {
        self.build_status = build_status;
    }

    fn add_dependency(&mut self, dependency: u32) {
        self.dependencies.push(dependency);
    }
}

fn main() {
    let mut pr_1 = PullRequest::new(1);
    let mut pr_2 = PullRequest::new(2);
    let mut dependency_monitor = DependencyMonitor::new();
    pr_1.set_build_status(true);
    println!("{:?}", pr_1);
}
