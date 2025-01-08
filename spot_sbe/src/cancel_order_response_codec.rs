use crate::*;

pub use decoder::CancelOrderResponseDecoder;
pub use encoder::CancelOrderResponseEncoder;

pub const SBE_BLOCK_LENGTH: u16 = 126;
pub const SBE_TEMPLATE_ID: u16 = 305;
pub const SBE_SCHEMA_ID: u16 = 2;
pub const SBE_SCHEMA_VERSION: u16 = 1;
pub const SBE_SEMANTIC_VERSION: &str = "5.2";

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct CancelOrderResponseEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for CancelOrderResponseEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for CancelOrderResponseEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> CancelOrderResponseEncoder<'a> {
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

        /// primitive field 'qtyExponent'
        /// - min value: -127
        /// - max value: 127
        /// - null value: -128
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 1
        /// - encodedLength: 1
        #[inline]
        pub fn qty_exponent(&mut self, value: i8) {
            let offset = self.offset + 1;
            self.get_buf_mut().put_i8_at(offset, value);
        }

        /// primitive field 'orderId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 2
        /// - encodedLength: 8
        #[inline]
        pub fn order_id(&mut self, value: i64) {
            let offset = self.offset + 2;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'orderListId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 10
        /// - encodedLength: 8
        #[inline]
        pub fn order_list_id(&mut self, value: i64) {
            let offset = self.offset + 10;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'transactTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 18
        /// - encodedLength: 8
        #[inline]
        pub fn transact_time(&mut self, value: i64) {
            let offset = self.offset + 18;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'price'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 26
        /// - encodedLength: 8
        #[inline]
        pub fn price(&mut self, value: i64) {
            let offset = self.offset + 26;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'origQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 34
        /// - encodedLength: 8
        #[inline]
        pub fn orig_qty(&mut self, value: i64) {
            let offset = self.offset + 34;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'executedQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 42
        /// - encodedLength: 8
        #[inline]
        pub fn executed_qty(&mut self, value: i64) {
            let offset = self.offset + 42;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'cummulativeQuoteQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 50
        /// - encodedLength: 8
        #[inline]
        pub fn cummulative_quote_qty(&mut self, value: i64) {
            let offset = self.offset + 50;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn status(&mut self, value: OrderStatus) {
            let offset = self.offset + 58;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&mut self, value: TimeInForce) {
            let offset = self.offset + 59;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_type(&mut self, value: OrderType) {
            let offset = self.offset + 60;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn side(&mut self, value: OrderSide) {
            let offset = self.offset + 61;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'stopPrice'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 62
        /// - encodedLength: 8
        #[inline]
        pub fn stop_price(&mut self, value: i64) {
            let offset = self.offset + 62;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'trailingDelta'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 70
        /// - encodedLength: 8
        #[inline]
        pub fn trailing_delta(&mut self, value: i64) {
            let offset = self.offset + 70;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'trailingTime'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 78
        /// - encodedLength: 8
        #[inline]
        pub fn trailing_time(&mut self, value: i64) {
            let offset = self.offset + 78;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'icebergQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 86
        /// - encodedLength: 8
        #[inline]
        pub fn iceberg_qty(&mut self, value: i64) {
            let offset = self.offset + 86;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'strategyId'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 94
        /// - encodedLength: 8
        #[inline]
        pub fn strategy_id(&mut self, value: i64) {
            let offset = self.offset + 94;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// primitive field 'strategyType'
        /// - min value: -2147483647
        /// - max value: 2147483647
        /// - null value: -2147483648
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 102
        /// - encodedLength: 4
        #[inline]
        pub fn strategy_type(&mut self, value: i32) {
            let offset = self.offset + 102;
            self.get_buf_mut().put_i32_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_capacity(&mut self, value: OrderCapacity) {
            let offset = self.offset + 106;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn working_floor(&mut self, value: Floor) {
            let offset = self.offset + 107;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn self_trade_prevention_mode(&mut self, value: SelfTradePreventionMode) {
            let offset = self.offset + 108;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'preventedQuantity'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 109
        /// - encodedLength: 8
        #[inline]
        pub fn prevented_quantity(&mut self, value: i64) {
            let offset = self.offset + 109;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn used_sor(&mut self, value: BoolEnum) {
            let offset = self.offset + 117;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'origQuoteOrderQty'
        /// - min value: -9223372036854775807
        /// - max value: 9223372036854775807
        /// - null value: -9223372036854775808
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 118
        /// - encodedLength: 8
        #[inline]
        pub fn orig_quote_order_qty(&mut self, value: i64) {
            let offset = self.offset + 118;
            self.get_buf_mut().put_i64_at(offset, value);
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn symbol(&mut self, value: &str) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 1 + data_length);
            self.get_buf_mut().put_u8_at(limit, data_length as u8);
            self.get_buf_mut().put_slice_at(limit + 1, value.as_bytes());
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn orig_client_order_id(&mut self, value: &str) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 1 + data_length);
            self.get_buf_mut().put_u8_at(limit, data_length as u8);
            self.get_buf_mut().put_slice_at(limit + 1, value.as_bytes());
        }

        /// VAR_DATA ENCODER - character encoding: 'UTF-8'
        #[inline]
        pub fn client_order_id(&mut self, value: &str) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 1 + data_length);
            self.get_buf_mut().put_u8_at(limit, data_length as u8);
            self.get_buf_mut().put_slice_at(limit + 1, value.as_bytes());
        }
    }
} // end encoder

pub mod decoder {
    use super::*;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct CancelOrderResponseDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for CancelOrderResponseDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for CancelOrderResponseDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> CancelOrderResponseDecoder<'a> {
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

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn price_exponent(&self) -> i8 {
            self.get_buf().get_i8_at(self.offset)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn qty_exponent(&self) -> i8 {
            self.get_buf().get_i8_at(self.offset + 1)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn order_id(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 2)
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808' }
        #[inline]
        pub fn order_list_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 10);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn transact_time(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 18)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn price(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 26)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn orig_qty(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 34)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn executed_qty(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 42)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn cummulative_quote_qty(&self) -> i64 {
            self.get_buf().get_i64_at(self.offset + 50)
        }

        /// REQUIRED enum
        #[inline]
        pub fn status(&self) -> OrderStatus {
            self.get_buf().get_u8_at(self.offset + 58).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn time_in_force(&self) -> TimeInForce {
            self.get_buf().get_u8_at(self.offset + 59).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_type(&self) -> OrderType {
            self.get_buf().get_u8_at(self.offset + 60).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn side(&self) -> OrderSide {
            self.get_buf().get_u8_at(self.offset + 61).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808' }
        #[inline]
        pub fn stop_price(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 62);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808' }
        #[inline]
        pub fn trailing_delta(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 70);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808' }
        #[inline]
        pub fn trailing_time(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 78);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808' }
        #[inline]
        pub fn iceberg_qty(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 86);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808' }
        #[inline]
        pub fn strategy_id(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 94);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: '-2147483648' }
        #[inline]
        pub fn strategy_type(&self) -> Option<i32> {
            let value = self.get_buf().get_i32_at(self.offset + 102);
            if value == -2147483648_i32 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn order_capacity(&self) -> OrderCapacity {
            self.get_buf().get_u8_at(self.offset + 106).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn working_floor(&self) -> Floor {
            self.get_buf().get_u8_at(self.offset + 107).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn self_trade_prevention_mode(&self) -> SelfTradePreventionMode {
            self.get_buf().get_u8_at(self.offset + 108).into()
        }

        /// primitive field - 'OPTIONAL' { null_value: '-9223372036854775808' }
        #[inline]
        pub fn prevented_quantity(&self) -> Option<i64> {
            let value = self.get_buf().get_i64_at(self.offset + 109);
            if value == -9223372036854775808_i64 {
                None
            } else {
                Some(value)
            }
        }

        /// REQUIRED enum
        #[inline]
        pub fn used_sor(&self) -> BoolEnum {
            self.get_buf().get_u8_at(self.offset + 117).into()
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn orig_quote_order_qty(&self) -> i64 {
            if self.acting_version > 0 && self.acting_version < 1 {
                return -9223372036854775808_i64;
            }

            self.get_buf().get_i64_at(self.offset + 118)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn symbol_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn symbol_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn orig_client_order_id_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn orig_client_order_id_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

        /// VAR_DATA DECODER - character encoding: 'UTF-8'
        #[inline]
        pub fn client_order_id_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn client_order_id_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }
    }
} // end decoder
