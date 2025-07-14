// @generated
impl serde::Serialize for BuyOrderBatchSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.buy_order_id != 0 {
            len += 1;
        }
        if self.batch_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.orderbook.v1alpha1.BuyOrderBatchSelector", len)?;
        if self.buy_order_id != 0 {
            struct_ser.serialize_field("buyOrderId", ToString::to_string(&self.buy_order_id).as_str())?;
        }
        if self.batch_id != 0 {
            struct_ser.serialize_field("batchId", ToString::to_string(&self.batch_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BuyOrderBatchSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "buy_order_id",
            "buyOrderId",
            "batch_id",
            "batchId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BuyOrderId,
            BatchId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "buyOrderId" | "buy_order_id" => Ok(GeneratedField::BuyOrderId),
                            "batchId" | "batch_id" => Ok(GeneratedField::BatchId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BuyOrderBatchSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.orderbook.v1alpha1.BuyOrderBatchSelector")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BuyOrderBatchSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut buy_order_id__ = None;
                let mut batch_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BuyOrderId => {
                            if buy_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buyOrderId"));
                            }
                            buy_order_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BatchId => {
                            if batch_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchId"));
                            }
                            batch_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BuyOrderBatchSelector {
                    buy_order_id: buy_order_id__.unwrap_or_default(),
                    batch_id: batch_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.orderbook.v1alpha1.BuyOrderBatchSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BuyOrderClassSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.buy_order_id != 0 {
            len += 1;
        }
        if self.class_id != 0 {
            len += 1;
        }
        if !self.project_location.is_empty() {
            len += 1;
        }
        if self.min_start_date.is_some() {
            len += 1;
        }
        if self.max_end_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.orderbook.v1alpha1.BuyOrderClassSelector", len)?;
        if self.buy_order_id != 0 {
            struct_ser.serialize_field("buyOrderId", ToString::to_string(&self.buy_order_id).as_str())?;
        }
        if self.class_id != 0 {
            struct_ser.serialize_field("classId", ToString::to_string(&self.class_id).as_str())?;
        }
        if !self.project_location.is_empty() {
            struct_ser.serialize_field("projectLocation", &self.project_location)?;
        }
        if let Some(v) = self.min_start_date.as_ref() {
            struct_ser.serialize_field("minStartDate", v)?;
        }
        if let Some(v) = self.max_end_date.as_ref() {
            struct_ser.serialize_field("maxEndDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BuyOrderClassSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "buy_order_id",
            "buyOrderId",
            "class_id",
            "classId",
            "project_location",
            "projectLocation",
            "min_start_date",
            "minStartDate",
            "max_end_date",
            "maxEndDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BuyOrderId,
            ClassId,
            ProjectLocation,
            MinStartDate,
            MaxEndDate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "buyOrderId" | "buy_order_id" => Ok(GeneratedField::BuyOrderId),
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
                            "projectLocation" | "project_location" => Ok(GeneratedField::ProjectLocation),
                            "minStartDate" | "min_start_date" => Ok(GeneratedField::MinStartDate),
                            "maxEndDate" | "max_end_date" => Ok(GeneratedField::MaxEndDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BuyOrderClassSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.orderbook.v1alpha1.BuyOrderClassSelector")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BuyOrderClassSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut buy_order_id__ = None;
                let mut class_id__ = None;
                let mut project_location__ = None;
                let mut min_start_date__ = None;
                let mut max_end_date__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BuyOrderId => {
                            if buy_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buyOrderId"));
                            }
                            buy_order_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProjectLocation => {
                            if project_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectLocation"));
                            }
                            project_location__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinStartDate => {
                            if min_start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minStartDate"));
                            }
                            min_start_date__ = map.next_value()?;
                        }
                        GeneratedField::MaxEndDate => {
                            if max_end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxEndDate"));
                            }
                            max_end_date__ = map.next_value()?;
                        }
                    }
                }
                Ok(BuyOrderClassSelector {
                    buy_order_id: buy_order_id__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                    project_location: project_location__.unwrap_or_default(),
                    min_start_date: min_start_date__,
                    max_end_date: max_end_date__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.orderbook.v1alpha1.BuyOrderClassSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BuyOrderProjectSelector {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.buy_order_id != 0 {
            len += 1;
        }
        if self.project_id != 0 {
            len += 1;
        }
        if self.min_start_date.is_some() {
            len += 1;
        }
        if self.max_end_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.orderbook.v1alpha1.BuyOrderProjectSelector", len)?;
        if self.buy_order_id != 0 {
            struct_ser.serialize_field("buyOrderId", ToString::to_string(&self.buy_order_id).as_str())?;
        }
        if self.project_id != 0 {
            struct_ser.serialize_field("projectId", ToString::to_string(&self.project_id).as_str())?;
        }
        if let Some(v) = self.min_start_date.as_ref() {
            struct_ser.serialize_field("minStartDate", v)?;
        }
        if let Some(v) = self.max_end_date.as_ref() {
            struct_ser.serialize_field("maxEndDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BuyOrderProjectSelector {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "buy_order_id",
            "buyOrderId",
            "project_id",
            "projectId",
            "min_start_date",
            "minStartDate",
            "max_end_date",
            "maxEndDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BuyOrderId,
            ProjectId,
            MinStartDate,
            MaxEndDate,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "buyOrderId" | "buy_order_id" => Ok(GeneratedField::BuyOrderId),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "minStartDate" | "min_start_date" => Ok(GeneratedField::MinStartDate),
                            "maxEndDate" | "max_end_date" => Ok(GeneratedField::MaxEndDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BuyOrderProjectSelector;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.orderbook.v1alpha1.BuyOrderProjectSelector")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BuyOrderProjectSelector, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut buy_order_id__ = None;
                let mut project_id__ = None;
                let mut min_start_date__ = None;
                let mut max_end_date__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BuyOrderId => {
                            if buy_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buyOrderId"));
                            }
                            buy_order_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MinStartDate => {
                            if min_start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minStartDate"));
                            }
                            min_start_date__ = map.next_value()?;
                        }
                        GeneratedField::MaxEndDate => {
                            if max_end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxEndDate"));
                            }
                            max_end_date__ = map.next_value()?;
                        }
                    }
                }
                Ok(BuyOrderProjectSelector {
                    buy_order_id: buy_order_id__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    min_start_date: min_start_date__,
                    max_end_date: max_end_date__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.orderbook.v1alpha1.BuyOrderProjectSelector", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BuyOrderSellOrderMatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.market_id != 0 {
            len += 1;
        }
        if self.buy_order_id != 0 {
            len += 1;
        }
        if self.sell_order_id != 0 {
            len += 1;
        }
        if self.bid_price_complement != 0 {
            len += 1;
        }
        if self.ask_price != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.orderbook.v1alpha1.BuyOrderSellOrderMatch", len)?;
        if self.market_id != 0 {
            struct_ser.serialize_field("marketId", ToString::to_string(&self.market_id).as_str())?;
        }
        if self.buy_order_id != 0 {
            struct_ser.serialize_field("buyOrderId", ToString::to_string(&self.buy_order_id).as_str())?;
        }
        if self.sell_order_id != 0 {
            struct_ser.serialize_field("sellOrderId", ToString::to_string(&self.sell_order_id).as_str())?;
        }
        if self.bid_price_complement != 0 {
            struct_ser.serialize_field("bidPriceComplement", &self.bid_price_complement)?;
        }
        if self.ask_price != 0 {
            struct_ser.serialize_field("askPrice", &self.ask_price)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BuyOrderSellOrderMatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "market_id",
            "marketId",
            "buy_order_id",
            "buyOrderId",
            "sell_order_id",
            "sellOrderId",
            "bid_price_complement",
            "bidPriceComplement",
            "ask_price",
            "askPrice",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MarketId,
            BuyOrderId,
            SellOrderId,
            BidPriceComplement,
            AskPrice,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "marketId" | "market_id" => Ok(GeneratedField::MarketId),
                            "buyOrderId" | "buy_order_id" => Ok(GeneratedField::BuyOrderId),
                            "sellOrderId" | "sell_order_id" => Ok(GeneratedField::SellOrderId),
                            "bidPriceComplement" | "bid_price_complement" => Ok(GeneratedField::BidPriceComplement),
                            "askPrice" | "ask_price" => Ok(GeneratedField::AskPrice),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BuyOrderSellOrderMatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.orderbook.v1alpha1.BuyOrderSellOrderMatch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BuyOrderSellOrderMatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut market_id__ = None;
                let mut buy_order_id__ = None;
                let mut sell_order_id__ = None;
                let mut bid_price_complement__ = None;
                let mut ask_price__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BuyOrderId => {
                            if buy_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buyOrderId"));
                            }
                            buy_order_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SellOrderId => {
                            if sell_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellOrderId"));
                            }
                            sell_order_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BidPriceComplement => {
                            if bid_price_complement__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bidPriceComplement"));
                            }
                            bid_price_complement__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AskPrice => {
                            if ask_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("askPrice"));
                            }
                            ask_price__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BuyOrderSellOrderMatch {
                    market_id: market_id__.unwrap_or_default(),
                    buy_order_id: buy_order_id__.unwrap_or_default(),
                    sell_order_id: sell_order_id__.unwrap_or_default(),
                    bid_price_complement: bid_price_complement__.unwrap_or_default(),
                    ask_price: ask_price__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.orderbook.v1alpha1.BuyOrderSellOrderMatch", FIELDS, GeneratedVisitor)
    }
}
