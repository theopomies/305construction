use super::task::Task;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{Error, ErrorKind, Read},
};

#[derive(Debug)]
pub struct Scheduler {
    tasks: HashMap<String, Task>,
    stack: Vec<String>,
    end: Option<u16>,
}

impl Scheduler {
    pub fn execute(&mut self) -> Result<(), Error> {
        let first_tasks = self.find_first_tasks()?;
        for task in first_tasks {
            self.visit_early(task);
        }
        while let Some(task) = self.stack.pop() {
            self.visit_early(task);
        }
        let end = self
            .tasks
            .iter()
            .try_fold(0, |acc, (name, task)| match task.earlier_start {
                Some(start) => Ok(std::cmp::max(task.duration + start, acc)),
                _ => Err(Error::new(
                    ErrorKind::Other,
                    format!("Circular dependency detected around {}", name),
                )),
            })?;
        self.end = Some(end);
        let last_tasks = self.find_last_tasks()?;
        for task in last_tasks {
            self.visit_last(task);
        }
        while let Some(task) = self.stack.pop() {
            self.visit_last(task);
        }
        Ok(())
    }

    fn find_first_tasks(&self) -> Result<Vec<String>, Error> {
        let v: Vec<String> = self
            .tasks
            .iter()
            .filter(|(_, task)| task.deps.is_empty())
            .map(|(name, _)| name.clone())
            .collect();
        match v.len() {
            0 => Err(Error::new(ErrorKind::Other, "No task without dependency.")),
            _ => Ok(v),
        }
    }

    fn find_last_tasks(&self) -> Result<Vec<String>, Error> {
        let v: Vec<String> = self
            .tasks
            .iter()
            .filter(|(_, task)| task.dependants.is_empty())
            .map(|(name, _)| name.clone())
            .collect();
        match v.len() {
            0 => Err(Error::new(ErrorKind::Other, "No task without dependency.")),
            _ => Ok(v),
        }
    }

    fn visit_early(&mut self, task_name: String) {
        let task = self.tasks.get(&task_name).unwrap();
        if task
            .deps
            .iter()
            .any(|task| self.tasks.get(task).unwrap().earlier_start.is_none())
        {
            return;
        }
        let dist = {
            let mut max = 0;
            for dep in &task.deps {
                let task = self.tasks.get(dep).unwrap();
                let dist = task.duration + task.earlier_start.unwrap();
                if dist > max {
                    max = dist;
                }
            }
            max
        };
        let task = self.tasks.get_mut(&task_name).unwrap();
        task.earlier_start = Some(dist);
        for task in &task.dependants {
            if !self.stack.contains(task) {
                self.stack.push(task.into());
            }
        }
    }

    fn visit_last(&mut self, task_name: String) {
        let task = self.tasks.get(&task_name).unwrap();
        if task
            .dependants
            .iter()
            .any(|task| self.tasks.get(task).unwrap().latest_start.is_none())
        {
            return;
        }
        let dist = task
            .dependants
            .iter()
            .map(|task| self.tasks.get(task).unwrap())
            .fold(None, |acc, dep| match acc {
                None => dep.latest_start,
                Some(value) => Some(std::cmp::min(value, dep.latest_start.unwrap())),
            })
            .map(|value| value - task.duration)
            .or_else(|| Some(self.end.unwrap() - task.duration));
        let task = self.tasks.get_mut(&task_name).unwrap();
        task.latest_start = dist;
        for task in &task.deps {
            if !self.stack.contains(task) {
                self.stack.push(task.into());
            }
        }
    }

    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            stack: vec![],
            end: None,
        }
    }

    fn try_add(mut self, name: &str, duration: u16, deps: &[&str]) -> Result<Self, Error> {
        if self.tasks.get(name).is_some() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Entry {} found multiple times.", name),
            ));
        }
        self.tasks.insert(name.into(), Task::new(duration, deps));
        Ok(self)
    }

    fn finalize(mut self) -> Result<Self, Error> {
        let mut deps_to_add: Vec<(String, String)> = vec![];
        for (name, task) in &mut self.tasks {
            for dep in &task.deps {
                deps_to_add.push((name.into(), dep.into()));
            }
        }
        for (name, dep) in deps_to_add {
            self.tasks
                .get_mut(&dep)
                .ok_or_else(|| {
                    Error::new(ErrorKind::Other, format!("Deps {} does not exist.", dep))
                })?
                .dependants
                .push(name);
        }
        Ok(self)
    }
}

impl TryFrom<&mut File> for Scheduler {
    type Error = Error;

    fn try_from(file: &mut File) -> Result<Self, Self::Error> {
        let mut buf = String::new();
        if file.read_to_string(&mut buf)? == 0 {
            Err(Error::new(ErrorKind::Other, "File empty."))
        } else {
            Self::try_from(buf)
        }
    }
}

impl TryFrom<String> for Scheduler {
    type Error = Error;
    fn try_from(file: String) -> Result<Self, Self::Error> {
        file.lines()
            .map(|l| l.split(';'))
            .try_fold(Self::new(), |scheduler, task| {
                match &task.collect::<Vec<_>>()[..] {
                    [name, _, duration, deps @ ..] if duration.parse::<u16>().is_ok() => {
                        Ok(scheduler.try_add(name, duration.parse().unwrap(), deps)?)
                    }
                    _ => Err(Error::new(ErrorKind::Other, "Invalid csv line.")),
                }
            })?
            .finalize()
    }
}

impl Display for Scheduler {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Total duration of construction: {} weeks\n",
            self.end.unwrap()
        )?;
        let mut tasks: Vec<_> = self.tasks.iter().collect();
        tasks.sort_by(|(n1, t1), (n2, t2)| {
            let s1 = t1.earlier_start.unwrap();
            let s2 = t2.earlier_start.unwrap();
            if s1 > s2 {
                return Ordering::Greater;
            }
            if s2 > s1 {
                return Ordering::Less;
            }
            let d1 = t1.duration;
            let d2 = t2.duration;
            if d1 > d2 {
                return Ordering::Greater;
            }
            if d2 > d1 {
                return Ordering::Less;
            }
            n1.cmp(n2)
        });
        for (name, task) in &tasks {
            let start = task.earlier_start.unwrap();
            let late_start = task.latest_start.unwrap();
            writeln!(
                f,
                "{} must begin {}",
                name,
                if start == late_start {
                    format!("at t={}", start)
                } else {
                    format!("between t={} and t={}", start, late_start)
                }
            )?;
        }
        writeln!(f)?;
        for (name, task) in tasks {
            let start = task.earlier_start.unwrap();
            let late_start = task.latest_start.unwrap();
            write!(f, "{}\t({})\t", name, late_start - start)?;
            write!(f, "{}", " ".repeat(start as usize))?;
            writeln!(f, "{}", "=".repeat(task.duration as usize))?;
        }
        Ok(())
    }
}
