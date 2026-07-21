use crate::dev::dt::{Fdt, FdtToken};

impl Fdt {
    pub fn cpu_count(&self) -> usize {
        let walker = self.lookup("/cpus");
        let mut depth = 0;
        let mut is_cpu = false;
        let mut is_enabled = true;
        let mut count = 0;

        for token in walker {
            match token {
                FdtToken::Node(_) => {
                    if depth == 0 {
                        is_cpu = false;
                        is_enabled = true;
                    }
                    depth += 1;
                }
                FdtToken::NodeEnd => {
                    if depth == 0 {
                        break;
                    }
                    if depth == 1 {
                        count += usize::from(is_cpu && is_enabled);
                    }
                    depth -= 1;
                }
                FdtToken::Prop { name, value } if depth == 1 => match name {
                    "device_type" => is_cpu = value.into_str() == Some("cpu"),
                    "status" => {
                        is_enabled = matches!(value.into_str(), Some("okay" | "ok"));
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        count
    }
}
