// Bitcoin transaction processing & database indexing daemon
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use tokio::{
    sync::mpsc,
    task::JoinHandle
};
use txlib::lnpbp::bitcoin::Block;
use super::*;

pub struct Service {
    config: Config,
    stats: Stats,
    pub task: JoinHandle<Result<!, Error>>
}

impl Service {
    pub fn init_and_run(config: Config, mut rx: mpsc::Receiver<Vec<Block>>) -> Result<Self, Error> {
        // TODO: Open daabase connection and init bulk parser

        let task = tokio::spawn(async move {
            while let Some(blocks) = rx.recv().await {
                // TODO: Process blocks
            }
            Err(Error::InputThreadDropped)
        });

        Ok(Self {
            config,
            stats: Stats::default(),
            task,
        })
    }
}