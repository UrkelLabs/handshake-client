impl HSClient {
    pub fn getblockcount(&mut self) -> Result<responses::GetBlockCount, Error> {
        self.call("getblockcount", &[])
    }
}
