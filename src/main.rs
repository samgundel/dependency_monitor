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

    fn set_build_status(&mut self, pr: u32, status: bool) {
        match self.pull_requests.get_mut(&pr) {
            Some(found_pr) => found_pr.monitored_pr.set_build_status(status, &found_pr),
            None => println!("No PR Found")
        }
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
        println!("Status Changed: {:?}", self.monitored_pr);
    }
}

impl<'a> Monitor for &'a PullRequestMonitor {
    fn notify_status_change(&self) {
        println!("Status Changed: {:?}", self.monitored_pr);
    }
}

impl<'a> Monitor for &'a mut PullRequestMonitor {
    fn notify_status_change(&self) {
        println!("Status Changed: {:?}", self.monitored_pr);
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

    fn set_build_status(&mut self, build_status: bool, monitor: &dyn Monitor) {
        if build_status != self.build_status {
            self.build_status = build_status;
            monitor.notify_status_change();
        }
    }

    fn add_dependency(&mut self, dependency: u32, monitor: &dyn Monitor) {
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
