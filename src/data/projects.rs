pub struct Project {
    pub title: &'static str,
    pub description: &'static str,
    pub languages: &'static [&'static str],
    pub github: Option<&'static str>,
}

pub const PROJECTS: &[Project] = &[
    Project {
        title: "swarm_ws",
        description: "robotic swarm maze exploration",
        languages: &["Python", "ROS 2", "Docker"],
        github: Some("https://github.com/RRohan4/swarm_ws"),
    },
    Project {
        title: "CRusTTY",
        description: "tui c interpreter from scratch",
        languages: &["Rust", "C"],
        github: Some("https://github.com/aicheye/crustty"),
    },
    Project {
        title: "Wunder RNN Challenge",
        description: "top 5% market prediction model",
        languages: &["Python", "ML", "Jupyter"],
        github: Some("https://github.com/aicheye/wundernn"),
    },
    Project {
        title: "aicheye's Combat Tagging",
        description: "minecraft mod w/ 1k+ downloads",
        languages: &["Java", "Gradle"],
        github: Some("https://github.com/aicheye/combat-tag"),
    },
    Project {
        title: "Bucket",
        description: "intuitive student dashboard",
        languages: &["TypeScript", "React", "GraphQL"],
        github: Some("https://github.com/aicheye/bucket"),
    },
    Project {
        title: "ezp2p Arcade",
        description: "p2p serverless minigames",
        languages: &["TypeScript", "Vite", "WebRTC"],
        github: Some("https://github.com/aicheye/ezp2p"),
    },
];
