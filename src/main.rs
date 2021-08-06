fn main() -> Result<(), Box<dyn std::error::Error>> {
    let t = std::thread::spawn(|| {
        let f = std::fs::File::open("/Users/clark/code/manix/Root_Vase.stl.torrent").unwrap();

        let options = manix::Options::default();
        let mut manix = manix::blocking_client(options);

        let out = Box::new(std::io::Cursor::new(vec![]));

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
