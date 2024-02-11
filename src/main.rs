use std::collections::HashMap;

pub trait PullRequestBuild {
    fn get_build_status(&self, pull_request: u32) -> bool;
}

#[derive(Debug)]
struct DependencyManager {
    pull_requests: HashMap<u32, PullRequestMonitor>
}

impl PullRequestBuild for DependencyManager {
    fn get_build_status(&self, pull_request: u32) -> bool {
        false
    }
}

impl DependencyManager {
    fn add_dependency(&mut self, source: u32, target: u32) {
       let mut pr = PullRequest::new(source);
       let mut monitor = PullRequestMonitor::new(pr);
       self.pull_requests.insert(source, monitor);
    }
    fn new() -> DependencyManager {
        DependencyManager {
            pull_requests: HashMap::new()
        }
    }
}


pub trait Monitor {
    fn notify_status_change(&self);
}

#[derive(Debug)]
struct PullRequestMonitor {
    monitored_pr: PullRequest,
}

impl Monitor for PullRequestMonitor {
    fn notify_status_change(&self) {
    }
}

impl PullRequestMonitor {
    fn new(monitored_pr: PullRequest) -> PullRequestMonitor {
        PullRequestMonitor {
            monitored_pr
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
            dependencies: Vec::new(),
        }
    }

    fn set_build_status(&mut self, build_status: bool, monitor: &mut dyn Monitor) {
        self.build_status = build_status;
    }

    fn add_dependency(&mut self, dependency: u32, monitor: &mut dyn Monitor) {
        self.dependencies.push(dependency);
    }
}

fn main() {
    let mut manager = DependencyManager::new();

    //let mut pr_1 = PullRequest::new(1);
    //let mut pr_2 = PullRequest::new(2);
    //let mut dependency_monitor = DependencyManager::new();
    //pr_1.set_build_status(true);
    //println!("{:?}", pr_1);
}
