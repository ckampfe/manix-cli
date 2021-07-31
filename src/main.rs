fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = std::fs::File::open("/home/clark/code/personal/bex/Root_Vase.stl.torrent")?;

    let options = manix::Options::default();
    let mut manix = manix::blocking_client(options);

    let out = Box::new(std::io::Cursor::new(vec![]));

    let torrent = manix.add_torrent(f, out).unwrap();

    println!("here");
    println!("announce_url: {}", torrent.get_announce_url());
    println!("info_hash: {}", torrent.get_info_hash_human());
    println!("peer_id: {}", torrent.get_peer_id().human_readable());

    let info_hash = torrent.get_info_hash_human();
    manix.start_torrent(&info_hash)?;

    Ok(())
}
