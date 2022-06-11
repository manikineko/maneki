
    use steamworks::AppId;
    use steamworks::Client;
    use steamworks::SingleClient;
    use steamworks::FriendFlags;
    use steamworks::PersonaStateChange;
    pub fn steam_init() -> (Client, SingleClient) {
        let appid = 1456420;
        let (client, single) = Client::init_app(appid).unwrap();
        let _cb = client.register_callback(|p: PersonaStateChange| {
            println!("Got callback: {:?}", p);
        });
       (client, single)
       
    }
