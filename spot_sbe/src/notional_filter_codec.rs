use crate::*;

pub use decoder::NotionalFilterDecoder;
pub use encoder::NotionalFilterEncoder;

pub const SBE_BLOCK_LENGTH: u16 = 23;
pub const SBE_TEMPLATE_ID: u16 = 6;
pub const SBE_SCHEMA_ID: u16 = 2;
pub const SBE_SCHEMA_VERSION: u16 = 1;
pub const SBE_SEMANTIC_VERSION: &str = "5.2";

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct NotionalFilterEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for NotionalFilterEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for NotionalFilterEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> NotionalFilterEncoder<'a> {
        pub fn wrap(mut self, buf: WriteBuf<'a>, offset: usize) -> Self {
            let limit = offset + SBE_BLOCK_LENGTH as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, offset: usize) -> MessageHeaderEncoder<Self> {
            let mut header = MessageHeaderEncoder::default().wrap(self, offset);
            header.block_length(SBE_BLOCK_LENGTH);
            header.template_id(SBE_TEMPLATE_ID);
            header.schema_id(SBE_SCHEMA_ID);
            header.version(SBE_SCHEMA_VERSION);
            header
        }

        // skipping CONSTANT enum 'filterType'

        /// primitive field 'priceExponent'
        /// - min value: -127
        /// - max value: 127
        /// - null value: -128
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 1
        #[inline]
        pub fn price_exponent(&mut self, value: i8) {
            let offset = self.offset;
            self.get_buf_mut().put_i8_at(offset, value);
        }

        /// primitive field 'minNotional'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 1
        /// - encodedLength: 8
        #[inline]
        pub fn min_notional(&mut self, value: i64) {
            let offset = self.offset + 1;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn apply_min_to_market(&mut self, value: BoolEnum) {
            let offset = self.offset + 9;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'maxNotional'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 10
        /// - encodedLength: 8
        #[inline]
        pub fn max_notional(&mut self, value: i64) {
            let offset = self.offset + 10;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn apply_max_to_market(&mut self, value: BoolEnum) {
            let offset = self.offset + 18;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'avgPriceMins'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 19
        /// - encodedLength: 4
        #[inline]
        pub fn avg_price_mins(&mut self, value: i32) {
            let offset = self.offset + 19;
            self.get_buf_mut().put_i32_at(offset, value);
        }
    }
} // end encoder

pub mod decoder {
    use super::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct NotionalFilterDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for NotionalFilterDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for NotionalFilterDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> NotionalFilterDecoder<'a> {
        pub fn wrap(
            mut self,
            buf: ReadBuf<'a>,
            offset: usize,
            acting_block_length: u16,
            acting_version: u16,
        ) -> Self {
            let limit = offset + acting_block_length as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self.acting_block_length = acting_block_length;
            self.acting_version = acting_version;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, mut header: MessageHeaderDecoder<ReadBuf<'a>>) -> Self {
            debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
            let acting_block_length = header.block_length();
            let acting_version = header.version();

            self.wrap(
                header.parent().unwrap(),
                message_header_codec::ENCODED_LENGTH,
                acting_block_length,
                acting_version,
            )
        }

        /// CONSTANT enum
        #[inline]
        pub fn filter_type(&self) -> FilterType {
            FilterType::Notional
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn price_exponent(&self) -> i8 {
            self.get_buf().get_i8_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn min_notional(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 1)
        }

        /// REQUIRED enum
        #[inline]
        pub fn apply_min_to_market(&self) -> BoolEnum {
            self.get_buf().get_u8_at(self.offset + 9).into()
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn max_notional(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 10)
        }

        /// REQUIRED enum
        #[inline]
        pub fn apply_max_to_market(&self) -> BoolEnum {
            self.get_buf().get_u8_at(self.offset + 18).into()
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn avg_price_mins(&self) -> i32 {
            self.get_buf().get_i32_at(self.offset + 19)
        }
    }
} // end decoder
