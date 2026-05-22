pub struct Job {
    pub title: &'static str,
    pub company: &'static str,
    pub description: &'static str,
    pub website: &'static str,
    pub location: &'static str,
    pub technologies: &'static [&'static str],
    pub dates: &'static [&'static str],
    pub current: bool,
}

pub const JOBS: &[Job] = &[
    Job {
        title: "Robotics Perception Intern",
        company: "moss",
        description: "data gen/ml pipelines & firmware for agbots",
        website: "https://moss.ag",
        location: "San Francisco, CA",
        technologies: &[],
        dates: &["2026.5", "present"],
        current: true,
    },
    Job {
        title: "Robotics SWE",
        company: "WATonomous",
        description: "prediction pipelines for autonomous vehicles",
        website: "https://watonomous.ca",
        location: "Waterloo, ON",
        technologies: &["C++", "ROS 2", "Docker", "Foxglove"],
        dates: &["2025.9", "2026.4"],
        current: false,
    },
];
