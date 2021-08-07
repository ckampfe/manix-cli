use std::env::args;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let t = std::thread::spawn(|| {
        tracing_subscriber::fmt::init();
        let mut a = args();
        let _program_name = a.next();
        let t = a.next().unwrap();
        let f = std::fs::File::open(t).unwrap();

        let options = manix::Options::default();
        let mut manix = manix::blocking_client(options);

        // let out = Box::new(std::io::Cursor::new(vec![]));
        let out = PathBuf::from("/home/clark/code/personal/manix-cli/Root_Vase.stl");

        let torrent = manix.add_torrent(f, out).unwrap();

        println!("here");
        println!("announce_url: {}", torrent.get_announce_url());
        println!("info_hash: {}", torrent.get_info_hash_human());
        println!("peer_id: {}", torrent.get_peer_id().human_readable());

        let info_hash = torrent.get_info_hash_human();
        println!("PRESTART-----------------");
        manix.start_torrent(&info_hash).unwrap();
        println!("STARTED---------------------------");
        std::thread::sleep(std::time::Duration::from_secs(10));
        println!("AFTER SLEEP------------------------");
        manix.pause_torrent(&info_hash).unwrap();
        std::thread::sleep(std::time::Duration::MAX)
    });

    t.join().unwrap();

    Ok(())
}
