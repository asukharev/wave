pub trait VecExt {
    fn push_u32_le(&mut self, value: u32) -> &mut Self;
    fn push_u32_be(&mut self, value: u32) -> &mut Self;

    fn push_u16_le(&mut self, value: u16) -> &mut Self;
    fn push_u16_be(&mut self, value: u16) -> &mut Self;
    fn push_i16_le(&mut self, value: i16) -> &mut Self;
    fn push_i16_be(&mut self, value: i16) -> &mut Self;

    fn push_all_ext(&mut self, src: &[u8]) -> &mut Self;
}

impl VecExt for Vec<u8> {
    fn push_u32_le(&mut self, value: u32) -> &mut Self {
        let mut cc: [u8; 4] = [0; 4];
        cc[3] = ((value <<  0) >> 24) as u8;
        cc[2] = ((value <<  8) >> 24) as u8;
        cc[1] = ((value << 16) >> 24) as u8;
        cc[0] = ((value << 24) >> 24) as u8;
        self.push_all_ext(&cc[..])
    }

    fn push_u32_be(&mut self, value: u32) -> &mut Self {
        let mut cc: [u8; 4] = [0; 4];
        cc[0] = ((value <<  0) >> 24) as u8;
        cc[1] = ((value <<  8) >> 24) as u8;
        cc[2] = ((value << 16) >> 24) as u8;
        cc[3] = ((value << 24) >> 24) as u8;
        self.push_all_ext(&cc[..])
    }

    fn push_u16_le(&mut self, value: u16) -> &mut Self {
        let mut cc: [u8; 2] = [0; 2];
        cc[1] = ((value <<  0) >> 8) as u8;
        cc[0] = ((value <<  8) >> 8) as u8;
        self.push_all_ext(&cc[..])
    }

    fn push_u16_be(&mut self, value: u16) -> &mut Self {
        let mut cc: [u8; 2] = [0; 2];
        cc[0] = ((value <<  0) >> 8) as u8;
        cc[1] = ((value <<  8) >> 8) as u8;
        self.push_all_ext(&cc[..])
    }

    fn push_i16_le(&mut self, value: i16) -> &mut Self {
        let mut cc: [u8; 2] = [0; 2];
        cc[1] = ((value <<  0) >> 8) as u8;
        cc[0] = ((value <<  8) >> 8) as u8;
        self.push_all_ext(&cc[..])
    }

    fn push_i16_be(&mut self, value: i16) -> &mut Self {
        let mut cc: [u8; 2] = [0; 2];
        cc[0] = ((value <<  0) >> 8) as u8;
        cc[1] = ((value <<  8) >> 8) as u8;
        self.push_all_ext(&cc[..])
    }

    fn push_all_ext(&mut self, src: &[u8]) -> &mut Self {
        for b in src {
            self.push(b.clone());
        }
        self
    }
}
