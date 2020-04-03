// Bitcoin protocol (BP) daemon node
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


use std::net::SocketAddr;

use crate::queryd::config::Config as MainConfig;


#[derive(Clone, PartialEq, Eq, Debug, Display)]
#[display_from(Debug)]
pub struct Config {
    pub socket_addr: SocketAddr
}

impl From<MainConfig> for Config {
    fn from(config: MainConfig) -> Self {
        Config {
            socket_addr: config.monitor_addr
        }
    }
}