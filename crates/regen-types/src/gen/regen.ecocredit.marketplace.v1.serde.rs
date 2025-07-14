// @generated
impl serde::Serialize for AllowDenomProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.denom.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.AllowDenomProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.denom.as_ref() {
            struct_ser.serialize_field("denom", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllowDenomProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "denom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            Denom,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "denom" => Ok(GeneratedField::Denom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AllowDenomProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.AllowDenomProposal")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AllowDenomProposal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = map_.next_value()?;
                        }
                    }
                }
                Ok(AllowDenomProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    denom: denom__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.AllowDenomProposal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AllowedDenom {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.bank_denom.is_empty() {
            len += 1;
        }
        if !self.display_denom.is_empty() {
            len += 1;
        }
        if self.exponent != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.AllowedDenom", len)?;
        if !self.bank_denom.is_empty() {
            struct_ser.serialize_field("bankDenom", &self.bank_denom)?;
        }
        if !self.display_denom.is_empty() {
            struct_ser.serialize_field("displayDenom", &self.display_denom)?;
        }
        if self.exponent != 0 {
            struct_ser.serialize_field("exponent", &self.exponent)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllowedDenom {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bank_denom",
            "bankDenom",
            "display_denom",
            "displayDenom",
            "exponent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BankDenom,
            DisplayDenom,
            Exponent,
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
                            "bankDenom" | "bank_denom" => Ok(GeneratedField::BankDenom),
                            "displayDenom" | "display_denom" => Ok(GeneratedField::DisplayDenom),
                            "exponent" => Ok(GeneratedField::Exponent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AllowedDenom;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.AllowedDenom")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<AllowedDenom, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bank_denom__ = None;
                let mut display_denom__ = None;
                let mut exponent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BankDenom => {
                            if bank_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bankDenom"));
                            }
                            bank_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayDenom => {
                            if display_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayDenom"));
                            }
                            display_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Exponent => {
                            if exponent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exponent"));
                            }
                            exponent__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AllowedDenom {
                    bank_denom: bank_denom__.unwrap_or_default(),
                    display_denom: display_denom__.unwrap_or_default(),
                    exponent: exponent__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.AllowedDenom", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventAllowDenom {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.denom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.EventAllowDenom", len)?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventAllowDenom {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "denom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
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
                            "denom" => Ok(GeneratedField::Denom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventAllowDenom;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.EventAllowDenom")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventAllowDenom, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventAllowDenom {
                    denom: denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.EventAllowDenom", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventBuyDirect {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sell_order_id != 0 {
            len += 1;
        }
        if !self.seller.is_empty() {
            len += 1;
        }
        if self.seller_fee_paid.is_some() {
            len += 1;
        }
        if !self.buyer.is_empty() {
            len += 1;
        }
        if self.buyer_fee_paid.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.EventBuyDirect", len)?;
        if self.sell_order_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sellOrderId", ToString::to_string(&self.sell_order_id).as_str())?;
        }
        if !self.seller.is_empty() {
            struct_ser.serialize_field("seller", &self.seller)?;
        }
        if let Some(v) = self.seller_fee_paid.as_ref() {
            struct_ser.serialize_field("sellerFeePaid", v)?;
        }
        if !self.buyer.is_empty() {
            struct_ser.serialize_field("buyer", &self.buyer)?;
        }
        if let Some(v) = self.buyer_fee_paid.as_ref() {
            struct_ser.serialize_field("buyerFeePaid", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventBuyDirect {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sell_order_id",
            "sellOrderId",
            "seller",
            "seller_fee_paid",
            "sellerFeePaid",
            "buyer",
            "buyer_fee_paid",
            "buyerFeePaid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SellOrderId,
            Seller,
            SellerFeePaid,
            Buyer,
            BuyerFeePaid,
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
                            "sellOrderId" | "sell_order_id" => Ok(GeneratedField::SellOrderId),
                            "seller" => Ok(GeneratedField::Seller),
                            "sellerFeePaid" | "seller_fee_paid" => Ok(GeneratedField::SellerFeePaid),
                            "buyer" => Ok(GeneratedField::Buyer),
                            "buyerFeePaid" | "buyer_fee_paid" => Ok(GeneratedField::BuyerFeePaid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventBuyDirect;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.EventBuyDirect")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventBuyDirect, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sell_order_id__ = None;
                let mut seller__ = None;
                let mut seller_fee_paid__ = None;
                let mut buyer__ = None;
                let mut buyer_fee_paid__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SellOrderId => {
                            if sell_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellOrderId"));
                            }
                            sell_order_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Seller => {
                            if seller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seller"));
                            }
                            seller__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SellerFeePaid => {
                            if seller_fee_paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellerFeePaid"));
                            }
                            seller_fee_paid__ = map_.next_value()?;
                        }
                        GeneratedField::Buyer => {
                            if buyer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buyer"));
                            }
                            buyer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BuyerFeePaid => {
                            if buyer_fee_paid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buyerFeePaid"));
                            }
                            buyer_fee_paid__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EventBuyDirect {
                    sell_order_id: sell_order_id__.unwrap_or_default(),
                    seller: seller__.unwrap_or_default(),
                    seller_fee_paid: seller_fee_paid__,
                    buyer: buyer__.unwrap_or_default(),
                    buyer_fee_paid: buyer_fee_paid__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.EventBuyDirect", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventCancelSellOrder {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sell_order_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.EventCancelSellOrder", len)?;
        if self.sell_order_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sellOrderId", ToString::to_string(&self.sell_order_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventCancelSellOrder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sell_order_id",
            "sellOrderId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SellOrderId,
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
                            "sellOrderId" | "sell_order_id" => Ok(GeneratedField::SellOrderId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventCancelSellOrder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.EventCancelSellOrder")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventCancelSellOrder, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sell_order_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SellOrderId => {
                            if sell_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellOrderId"));
                            }
                            sell_order_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(EventCancelSellOrder {
                    sell_order_id: sell_order_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.EventCancelSellOrder", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventRemoveAllowedDenom {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.denom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.EventRemoveAllowedDenom", len)?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventRemoveAllowedDenom {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "denom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
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
                            "denom" => Ok(GeneratedField::Denom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventRemoveAllowedDenom;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.EventRemoveAllowedDenom")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventRemoveAllowedDenom, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventRemoveAllowedDenom {
                    denom: denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.EventRemoveAllowedDenom", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventSell {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sell_order_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.EventSell", len)?;
        if self.sell_order_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sellOrderId", ToString::to_string(&self.sell_order_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventSell {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sell_order_id",
            "sellOrderId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SellOrderId,
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
                            "sellOrderId" | "sell_order_id" => Ok(GeneratedField::SellOrderId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventSell;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.EventSell")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventSell, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sell_order_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SellOrderId => {
                            if sell_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellOrderId"));
                            }
                            sell_order_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(EventSell {
                    sell_order_id: sell_order_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.EventSell", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventUpdateSellOrder {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sell_order_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.EventUpdateSellOrder", len)?;
        if self.sell_order_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sellOrderId", ToString::to_string(&self.sell_order_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventUpdateSellOrder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sell_order_id",
            "sellOrderId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SellOrderId,
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
                            "sellOrderId" | "sell_order_id" => Ok(GeneratedField::SellOrderId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventUpdateSellOrder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.EventUpdateSellOrder")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventUpdateSellOrder, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sell_order_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SellOrderId => {
                            if sell_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellOrderId"));
                            }
                            sell_order_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(EventUpdateSellOrder {
                    sell_order_id: sell_order_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.EventUpdateSellOrder", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FeeParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.buyer_percentage_fee.is_empty() {
            len += 1;
        }
        if !self.seller_percentage_fee.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.FeeParams", len)?;
        if !self.buyer_percentage_fee.is_empty() {
            struct_ser.serialize_field("buyerPercentageFee", &self.buyer_percentage_fee)?;
        }
        if !self.seller_percentage_fee.is_empty() {
            struct_ser.serialize_field("sellerPercentageFee", &self.seller_percentage_fee)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FeeParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "buyer_percentage_fee",
            "buyerPercentageFee",
            "seller_percentage_fee",
            "sellerPercentageFee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BuyerPercentageFee,
            SellerPercentageFee,
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
                            "buyerPercentageFee" | "buyer_percentage_fee" => Ok(GeneratedField::BuyerPercentageFee),
                            "sellerPercentageFee" | "seller_percentage_fee" => Ok(GeneratedField::SellerPercentageFee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FeeParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.FeeParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FeeParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut buyer_percentage_fee__ = None;
                let mut seller_percentage_fee__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BuyerPercentageFee => {
                            if buyer_percentage_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buyerPercentageFee"));
                            }
                            buyer_percentage_fee__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SellerPercentageFee => {
                            if seller_percentage_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellerPercentageFee"));
                            }
                            seller_percentage_fee__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(FeeParams {
                    buyer_percentage_fee: buyer_percentage_fee__.unwrap_or_default(),
                    seller_percentage_fee: seller_percentage_fee__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.FeeParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Market {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.credit_type_abbrev.is_empty() {
            len += 1;
        }
        if !self.bank_denom.is_empty() {
            len += 1;
        }
        if self.precision_modifier != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.Market", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if !self.credit_type_abbrev.is_empty() {
            struct_ser.serialize_field("creditTypeAbbrev", &self.credit_type_abbrev)?;
        }
        if !self.bank_denom.is_empty() {
            struct_ser.serialize_field("bankDenom", &self.bank_denom)?;
        }
        if self.precision_modifier != 0 {
            struct_ser.serialize_field("precisionModifier", &self.precision_modifier)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Market {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "credit_type_abbrev",
            "creditTypeAbbrev",
            "bank_denom",
            "bankDenom",
            "precision_modifier",
            "precisionModifier",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            CreditTypeAbbrev,
            BankDenom,
            PrecisionModifier,
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
                            "id" => Ok(GeneratedField::Id),
                            "creditTypeAbbrev" | "credit_type_abbrev" => Ok(GeneratedField::CreditTypeAbbrev),
                            "bankDenom" | "bank_denom" => Ok(GeneratedField::BankDenom),
                            "precisionModifier" | "precision_modifier" => Ok(GeneratedField::PrecisionModifier),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Market;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.Market")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Market, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut credit_type_abbrev__ = None;
                let mut bank_denom__ = None;
                let mut precision_modifier__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CreditTypeAbbrev => {
                            if credit_type_abbrev__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditTypeAbbrev"));
                            }
                            credit_type_abbrev__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BankDenom => {
                            if bank_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bankDenom"));
                            }
                            bank_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrecisionModifier => {
                            if precision_modifier__.is_some() {
                                return Err(serde::de::Error::duplicate_field("precisionModifier"));
                            }
                            precision_modifier__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Market {
                    id: id__.unwrap_or_default(),
                    credit_type_abbrev: credit_type_abbrev__.unwrap_or_default(),
                    bank_denom: bank_denom__.unwrap_or_default(),
                    precision_modifier: precision_modifier__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.Market", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddAllowedDenom {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.bank_denom.is_empty() {
            len += 1;
        }
        if !self.display_denom.is_empty() {
            len += 1;
        }
        if self.exponent != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgAddAllowedDenom", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.bank_denom.is_empty() {
            struct_ser.serialize_field("bankDenom", &self.bank_denom)?;
        }
        if !self.display_denom.is_empty() {
            struct_ser.serialize_field("displayDenom", &self.display_denom)?;
        }
        if self.exponent != 0 {
            struct_ser.serialize_field("exponent", &self.exponent)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddAllowedDenom {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "bank_denom",
            "bankDenom",
            "display_denom",
            "displayDenom",
            "exponent",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            BankDenom,
            DisplayDenom,
            Exponent,
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
                            "authority" => Ok(GeneratedField::Authority),
                            "bankDenom" | "bank_denom" => Ok(GeneratedField::BankDenom),
                            "displayDenom" | "display_denom" => Ok(GeneratedField::DisplayDenom),
                            "exponent" => Ok(GeneratedField::Exponent),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddAllowedDenom;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgAddAllowedDenom")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgAddAllowedDenom, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut bank_denom__ = None;
                let mut display_denom__ = None;
                let mut exponent__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BankDenom => {
                            if bank_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bankDenom"));
                            }
                            bank_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisplayDenom => {
                            if display_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayDenom"));
                            }
                            display_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Exponent => {
                            if exponent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exponent"));
                            }
                            exponent__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MsgAddAllowedDenom {
                    authority: authority__.unwrap_or_default(),
                    bank_denom: bank_denom__.unwrap_or_default(),
                    display_denom: display_denom__.unwrap_or_default(),
                    exponent: exponent__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgAddAllowedDenom", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddAllowedDenomResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgAddAllowedDenomResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddAllowedDenomResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddAllowedDenomResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgAddAllowedDenomResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgAddAllowedDenomResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAddAllowedDenomResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgAddAllowedDenomResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgBuyDirect {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.buyer.is_empty() {
            len += 1;
        }
        if !self.orders.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgBuyDirect", len)?;
        if !self.buyer.is_empty() {
            struct_ser.serialize_field("buyer", &self.buyer)?;
        }
        if !self.orders.is_empty() {
            struct_ser.serialize_field("orders", &self.orders)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgBuyDirect {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "buyer",
            "orders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Buyer,
            Orders,
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
                            "buyer" => Ok(GeneratedField::Buyer),
                            "orders" => Ok(GeneratedField::Orders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgBuyDirect;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgBuyDirect")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgBuyDirect, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut buyer__ = None;
                let mut orders__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Buyer => {
                            if buyer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("buyer"));
                            }
                            buyer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Orders => {
                            if orders__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orders"));
                            }
                            orders__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgBuyDirect {
                    buyer: buyer__.unwrap_or_default(),
                    orders: orders__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgBuyDirect", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for msg_buy_direct::Order {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sell_order_id != 0 {
            len += 1;
        }
        if !self.quantity.is_empty() {
            len += 1;
        }
        if self.bid_price.is_some() {
            len += 1;
        }
        if self.disable_auto_retire {
            len += 1;
        }
        if !self.retirement_jurisdiction.is_empty() {
            len += 1;
        }
        if !self.retirement_reason.is_empty() {
            len += 1;
        }
        if self.max_fee_amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgBuyDirect.Order", len)?;
        if self.sell_order_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sellOrderId", ToString::to_string(&self.sell_order_id).as_str())?;
        }
        if !self.quantity.is_empty() {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if let Some(v) = self.bid_price.as_ref() {
            struct_ser.serialize_field("bidPrice", v)?;
        }
        if self.disable_auto_retire {
            struct_ser.serialize_field("disableAutoRetire", &self.disable_auto_retire)?;
        }
        if !self.retirement_jurisdiction.is_empty() {
            struct_ser.serialize_field("retirementJurisdiction", &self.retirement_jurisdiction)?;
        }
        if !self.retirement_reason.is_empty() {
            struct_ser.serialize_field("retirementReason", &self.retirement_reason)?;
        }
        if let Some(v) = self.max_fee_amount.as_ref() {
            struct_ser.serialize_field("maxFeeAmount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for msg_buy_direct::Order {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sell_order_id",
            "sellOrderId",
            "quantity",
            "bid_price",
            "bidPrice",
            "disable_auto_retire",
            "disableAutoRetire",
            "retirement_jurisdiction",
            "retirementJurisdiction",
            "retirement_reason",
            "retirementReason",
            "max_fee_amount",
            "maxFeeAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SellOrderId,
            Quantity,
            BidPrice,
            DisableAutoRetire,
            RetirementJurisdiction,
            RetirementReason,
            MaxFeeAmount,
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
                            "sellOrderId" | "sell_order_id" => Ok(GeneratedField::SellOrderId),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "bidPrice" | "bid_price" => Ok(GeneratedField::BidPrice),
                            "disableAutoRetire" | "disable_auto_retire" => Ok(GeneratedField::DisableAutoRetire),
                            "retirementJurisdiction" | "retirement_jurisdiction" => Ok(GeneratedField::RetirementJurisdiction),
                            "retirementReason" | "retirement_reason" => Ok(GeneratedField::RetirementReason),
                            "maxFeeAmount" | "max_fee_amount" => Ok(GeneratedField::MaxFeeAmount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = msg_buy_direct::Order;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgBuyDirect.Order")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<msg_buy_direct::Order, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sell_order_id__ = None;
                let mut quantity__ = None;
                let mut bid_price__ = None;
                let mut disable_auto_retire__ = None;
                let mut retirement_jurisdiction__ = None;
                let mut retirement_reason__ = None;
                let mut max_fee_amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SellOrderId => {
                            if sell_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellOrderId"));
                            }
                            sell_order_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BidPrice => {
                            if bid_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bidPrice"));
                            }
                            bid_price__ = map_.next_value()?;
                        }
                        GeneratedField::DisableAutoRetire => {
                            if disable_auto_retire__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableAutoRetire"));
                            }
                            disable_auto_retire__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetirementJurisdiction => {
                            if retirement_jurisdiction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retirementJurisdiction"));
                            }
                            retirement_jurisdiction__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetirementReason => {
                            if retirement_reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retirementReason"));
                            }
                            retirement_reason__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MaxFeeAmount => {
                            if max_fee_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxFeeAmount"));
                            }
                            max_fee_amount__ = map_.next_value()?;
                        }
                    }
                }
                Ok(msg_buy_direct::Order {
                    sell_order_id: sell_order_id__.unwrap_or_default(),
                    quantity: quantity__.unwrap_or_default(),
                    bid_price: bid_price__,
                    disable_auto_retire: disable_auto_retire__.unwrap_or_default(),
                    retirement_jurisdiction: retirement_jurisdiction__.unwrap_or_default(),
                    retirement_reason: retirement_reason__.unwrap_or_default(),
                    max_fee_amount: max_fee_amount__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgBuyDirect.Order", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgBuyDirectResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgBuyDirectResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgBuyDirectResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgBuyDirectResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgBuyDirectResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgBuyDirectResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgBuyDirectResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgBuyDirectResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCancelSellOrder {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.seller.is_empty() {
            len += 1;
        }
        if self.sell_order_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgCancelSellOrder", len)?;
        if !self.seller.is_empty() {
            struct_ser.serialize_field("seller", &self.seller)?;
        }
        if self.sell_order_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sellOrderId", ToString::to_string(&self.sell_order_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCancelSellOrder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "seller",
            "sell_order_id",
            "sellOrderId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seller,
            SellOrderId,
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
                            "seller" => Ok(GeneratedField::Seller),
                            "sellOrderId" | "sell_order_id" => Ok(GeneratedField::SellOrderId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCancelSellOrder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgCancelSellOrder")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgCancelSellOrder, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut seller__ = None;
                let mut sell_order_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Seller => {
                            if seller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seller"));
                            }
                            seller__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SellOrderId => {
                            if sell_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellOrderId"));
                            }
                            sell_order_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MsgCancelSellOrder {
                    seller: seller__.unwrap_or_default(),
                    sell_order_id: sell_order_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgCancelSellOrder", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCancelSellOrderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgCancelSellOrderResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCancelSellOrderResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCancelSellOrderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgCancelSellOrderResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgCancelSellOrderResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCancelSellOrderResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgCancelSellOrderResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgGovSendFromFeePool {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.coins.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgGovSendFromFeePool", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.coins.is_empty() {
            struct_ser.serialize_field("coins", &self.coins)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgGovSendFromFeePool {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "recipient",
            "coins",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Recipient,
            Coins,
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
                            "authority" => Ok(GeneratedField::Authority),
                            "recipient" => Ok(GeneratedField::Recipient),
                            "coins" => Ok(GeneratedField::Coins),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgGovSendFromFeePool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgGovSendFromFeePool")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgGovSendFromFeePool, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut recipient__ = None;
                let mut coins__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Coins => {
                            if coins__.is_some() {
                                return Err(serde::de::Error::duplicate_field("coins"));
                            }
                            coins__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgGovSendFromFeePool {
                    authority: authority__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    coins: coins__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgGovSendFromFeePool", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgGovSendFromFeePoolResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgGovSendFromFeePoolResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgGovSendFromFeePoolResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgGovSendFromFeePoolResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgGovSendFromFeePoolResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgGovSendFromFeePoolResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgGovSendFromFeePoolResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgGovSendFromFeePoolResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgGovSetFeeParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if self.fees.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgGovSetFeeParams", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.fees.as_ref() {
            struct_ser.serialize_field("fees", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgGovSetFeeParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "fees",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Fees,
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
                            "authority" => Ok(GeneratedField::Authority),
                            "fees" => Ok(GeneratedField::Fees),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgGovSetFeeParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgGovSetFeeParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgGovSetFeeParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut fees__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fees => {
                            if fees__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fees"));
                            }
                            fees__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgGovSetFeeParams {
                    authority: authority__.unwrap_or_default(),
                    fees: fees__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgGovSetFeeParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgGovSetFeeParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgGovSetFeeParamsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgGovSetFeeParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgGovSetFeeParamsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgGovSetFeeParamsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgGovSetFeeParamsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgGovSetFeeParamsResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgGovSetFeeParamsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRemoveAllowedDenom {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.denom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgRemoveAllowedDenom", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveAllowedDenom {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "denom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Denom,
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
                            "authority" => Ok(GeneratedField::Authority),
                            "denom" => Ok(GeneratedField::Denom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemoveAllowedDenom;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgRemoveAllowedDenom")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgRemoveAllowedDenom, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRemoveAllowedDenom {
                    authority: authority__.unwrap_or_default(),
                    denom: denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgRemoveAllowedDenom", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRemoveAllowedDenomResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgRemoveAllowedDenomResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveAllowedDenomResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemoveAllowedDenomResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgRemoveAllowedDenomResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgRemoveAllowedDenomResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemoveAllowedDenomResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgRemoveAllowedDenomResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgSell {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.seller.is_empty() {
            len += 1;
        }
        if !self.orders.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgSell", len)?;
        if !self.seller.is_empty() {
            struct_ser.serialize_field("seller", &self.seller)?;
        }
        if !self.orders.is_empty() {
            struct_ser.serialize_field("orders", &self.orders)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSell {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "seller",
            "orders",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seller,
            Orders,
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
                            "seller" => Ok(GeneratedField::Seller),
                            "orders" => Ok(GeneratedField::Orders),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSell;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgSell")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgSell, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut seller__ = None;
                let mut orders__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Seller => {
                            if seller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seller"));
                            }
                            seller__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Orders => {
                            if orders__.is_some() {
                                return Err(serde::de::Error::duplicate_field("orders"));
                            }
                            orders__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgSell {
                    seller: seller__.unwrap_or_default(),
                    orders: orders__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgSell", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for msg_sell::Order {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        if !self.quantity.is_empty() {
            len += 1;
        }
        if self.ask_price.is_some() {
            len += 1;
        }
        if self.disable_auto_retire {
            len += 1;
        }
        if self.expiration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgSell.Order", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.quantity.is_empty() {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if let Some(v) = self.ask_price.as_ref() {
            struct_ser.serialize_field("askPrice", v)?;
        }
        if self.disable_auto_retire {
            struct_ser.serialize_field("disableAutoRetire", &self.disable_auto_retire)?;
        }
        if let Some(v) = self.expiration.as_ref() {
            struct_ser.serialize_field("expiration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for msg_sell::Order {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_denom",
            "batchDenom",
            "quantity",
            "ask_price",
            "askPrice",
            "disable_auto_retire",
            "disableAutoRetire",
            "expiration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchDenom,
            Quantity,
            AskPrice,
            DisableAutoRetire,
            Expiration,
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
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "askPrice" | "ask_price" => Ok(GeneratedField::AskPrice),
                            "disableAutoRetire" | "disable_auto_retire" => Ok(GeneratedField::DisableAutoRetire),
                            "expiration" => Ok(GeneratedField::Expiration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = msg_sell::Order;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgSell.Order")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<msg_sell::Order, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                let mut quantity__ = None;
                let mut ask_price__ = None;
                let mut disable_auto_retire__ = None;
                let mut expiration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AskPrice => {
                            if ask_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("askPrice"));
                            }
                            ask_price__ = map_.next_value()?;
                        }
                        GeneratedField::DisableAutoRetire => {
                            if disable_auto_retire__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableAutoRetire"));
                            }
                            disable_auto_retire__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Expiration => {
                            if expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiration"));
                            }
                            expiration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(msg_sell::Order {
                    batch_denom: batch_denom__.unwrap_or_default(),
                    quantity: quantity__.unwrap_or_default(),
                    ask_price: ask_price__,
                    disable_auto_retire: disable_auto_retire__.unwrap_or_default(),
                    expiration: expiration__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgSell.Order", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgSellResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sell_order_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgSellResponse", len)?;
        if !self.sell_order_ids.is_empty() {
            struct_ser.serialize_field("sellOrderIds", &self.sell_order_ids.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSellResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sell_order_ids",
            "sellOrderIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SellOrderIds,
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
                            "sellOrderIds" | "sell_order_ids" => Ok(GeneratedField::SellOrderIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSellResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgSellResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgSellResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sell_order_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SellOrderIds => {
                            if sell_order_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellOrderIds"));
                            }
                            sell_order_ids__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(MsgSellResponse {
                    sell_order_ids: sell_order_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgSellResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateSellOrders {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.seller.is_empty() {
            len += 1;
        }
        if !self.updates.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgUpdateSellOrders", len)?;
        if !self.seller.is_empty() {
            struct_ser.serialize_field("seller", &self.seller)?;
        }
        if !self.updates.is_empty() {
            struct_ser.serialize_field("updates", &self.updates)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateSellOrders {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "seller",
            "updates",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seller,
            Updates,
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
                            "seller" => Ok(GeneratedField::Seller),
                            "updates" => Ok(GeneratedField::Updates),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateSellOrders;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgUpdateSellOrders")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateSellOrders, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut seller__ = None;
                let mut updates__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Seller => {
                            if seller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seller"));
                            }
                            seller__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Updates => {
                            if updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updates"));
                            }
                            updates__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateSellOrders {
                    seller: seller__.unwrap_or_default(),
                    updates: updates__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgUpdateSellOrders", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for msg_update_sell_orders::Update {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sell_order_id != 0 {
            len += 1;
        }
        if !self.new_quantity.is_empty() {
            len += 1;
        }
        if self.new_ask_price.is_some() {
            len += 1;
        }
        if self.disable_auto_retire {
            len += 1;
        }
        if self.new_expiration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgUpdateSellOrders.Update", len)?;
        if self.sell_order_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sellOrderId", ToString::to_string(&self.sell_order_id).as_str())?;
        }
        if !self.new_quantity.is_empty() {
            struct_ser.serialize_field("newQuantity", &self.new_quantity)?;
        }
        if let Some(v) = self.new_ask_price.as_ref() {
            struct_ser.serialize_field("newAskPrice", v)?;
        }
        if self.disable_auto_retire {
            struct_ser.serialize_field("disableAutoRetire", &self.disable_auto_retire)?;
        }
        if let Some(v) = self.new_expiration.as_ref() {
            struct_ser.serialize_field("newExpiration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for msg_update_sell_orders::Update {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sell_order_id",
            "sellOrderId",
            "new_quantity",
            "newQuantity",
            "new_ask_price",
            "newAskPrice",
            "disable_auto_retire",
            "disableAutoRetire",
            "new_expiration",
            "newExpiration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SellOrderId,
            NewQuantity,
            NewAskPrice,
            DisableAutoRetire,
            NewExpiration,
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
                            "sellOrderId" | "sell_order_id" => Ok(GeneratedField::SellOrderId),
                            "newQuantity" | "new_quantity" => Ok(GeneratedField::NewQuantity),
                            "newAskPrice" | "new_ask_price" => Ok(GeneratedField::NewAskPrice),
                            "disableAutoRetire" | "disable_auto_retire" => Ok(GeneratedField::DisableAutoRetire),
                            "newExpiration" | "new_expiration" => Ok(GeneratedField::NewExpiration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = msg_update_sell_orders::Update;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgUpdateSellOrders.Update")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<msg_update_sell_orders::Update, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sell_order_id__ = None;
                let mut new_quantity__ = None;
                let mut new_ask_price__ = None;
                let mut disable_auto_retire__ = None;
                let mut new_expiration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SellOrderId => {
                            if sell_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellOrderId"));
                            }
                            sell_order_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NewQuantity => {
                            if new_quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newQuantity"));
                            }
                            new_quantity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewAskPrice => {
                            if new_ask_price__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newAskPrice"));
                            }
                            new_ask_price__ = map_.next_value()?;
                        }
                        GeneratedField::DisableAutoRetire => {
                            if disable_auto_retire__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableAutoRetire"));
                            }
                            disable_auto_retire__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewExpiration => {
                            if new_expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newExpiration"));
                            }
                            new_expiration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(msg_update_sell_orders::Update {
                    sell_order_id: sell_order_id__.unwrap_or_default(),
                    new_quantity: new_quantity__.unwrap_or_default(),
                    new_ask_price: new_ask_price__,
                    disable_auto_retire: disable_auto_retire__.unwrap_or_default(),
                    new_expiration: new_expiration__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgUpdateSellOrders.Update", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateSellOrdersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.MsgUpdateSellOrdersResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateSellOrdersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateSellOrdersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.MsgUpdateSellOrdersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateSellOrdersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateSellOrdersResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.MsgUpdateSellOrdersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAllowedDenomsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.QueryAllowedDenomsRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllowedDenomsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllowedDenomsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.QueryAllowedDenomsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryAllowedDenomsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAllowedDenomsRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.QueryAllowedDenomsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAllowedDenomsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.allowed_denoms.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.QueryAllowedDenomsResponse", len)?;
        if !self.allowed_denoms.is_empty() {
            struct_ser.serialize_field("allowedDenoms", &self.allowed_denoms)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllowedDenomsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_denoms",
            "allowedDenoms",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedDenoms,
            Pagination,
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
                            "allowedDenoms" | "allowed_denoms" => Ok(GeneratedField::AllowedDenoms),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllowedDenomsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.QueryAllowedDenomsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryAllowedDenomsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_denoms__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AllowedDenoms => {
                            if allowed_denoms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedDenoms"));
                            }
                            allowed_denoms__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryAllowedDenomsResponse {
                    allowed_denoms: allowed_denoms__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.QueryAllowedDenomsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuerySellOrderRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sell_order_id != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrderRequest", len)?;
        if self.sell_order_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("sellOrderId", ToString::to_string(&self.sell_order_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuerySellOrderRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sell_order_id",
            "sellOrderId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SellOrderId,
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
                            "sellOrderId" | "sell_order_id" => Ok(GeneratedField::SellOrderId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuerySellOrderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.QuerySellOrderRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuerySellOrderRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sell_order_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SellOrderId => {
                            if sell_order_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellOrderId"));
                            }
                            sell_order_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QuerySellOrderRequest {
                    sell_order_id: sell_order_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuerySellOrderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.sell_order.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrderResponse", len)?;
        if let Some(v) = self.sell_order.as_ref() {
            struct_ser.serialize_field("sellOrder", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuerySellOrderResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sell_order",
            "sellOrder",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SellOrder,
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
                            "sellOrder" | "sell_order" => Ok(GeneratedField::SellOrder),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuerySellOrderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.QuerySellOrderResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuerySellOrderResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sell_order__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SellOrder => {
                            if sell_order__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellOrder"));
                            }
                            sell_order__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuerySellOrderResponse {
                    sell_order: sell_order__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrderResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuerySellOrdersByBatchRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrdersByBatchRequest", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuerySellOrdersByBatchRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_denom",
            "batchDenom",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchDenom,
            Pagination,
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
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuerySellOrdersByBatchRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.QuerySellOrdersByBatchRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuerySellOrdersByBatchRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuerySellOrdersByBatchRequest {
                    batch_denom: batch_denom__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrdersByBatchRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuerySellOrdersByBatchResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sell_orders.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrdersByBatchResponse", len)?;
        if !self.sell_orders.is_empty() {
            struct_ser.serialize_field("sellOrders", &self.sell_orders)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuerySellOrdersByBatchResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sell_orders",
            "sellOrders",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SellOrders,
            Pagination,
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
                            "sellOrders" | "sell_orders" => Ok(GeneratedField::SellOrders),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuerySellOrdersByBatchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.QuerySellOrdersByBatchResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuerySellOrdersByBatchResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sell_orders__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SellOrders => {
                            if sell_orders__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellOrders"));
                            }
                            sell_orders__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuerySellOrdersByBatchResponse {
                    sell_orders: sell_orders__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrdersByBatchResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuerySellOrdersBySellerRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.seller.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrdersBySellerRequest", len)?;
        if !self.seller.is_empty() {
            struct_ser.serialize_field("seller", &self.seller)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuerySellOrdersBySellerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "seller",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Seller,
            Pagination,
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
                            "seller" => Ok(GeneratedField::Seller),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuerySellOrdersBySellerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.QuerySellOrdersBySellerRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuerySellOrdersBySellerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut seller__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Seller => {
                            if seller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seller"));
                            }
                            seller__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuerySellOrdersBySellerRequest {
                    seller: seller__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrdersBySellerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuerySellOrdersBySellerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sell_orders.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrdersBySellerResponse", len)?;
        if !self.sell_orders.is_empty() {
            struct_ser.serialize_field("sellOrders", &self.sell_orders)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuerySellOrdersBySellerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sell_orders",
            "sellOrders",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SellOrders,
            Pagination,
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
                            "sellOrders" | "sell_orders" => Ok(GeneratedField::SellOrders),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuerySellOrdersBySellerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.QuerySellOrdersBySellerResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuerySellOrdersBySellerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sell_orders__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SellOrders => {
                            if sell_orders__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellOrders"));
                            }
                            sell_orders__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuerySellOrdersBySellerResponse {
                    sell_orders: sell_orders__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrdersBySellerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuerySellOrdersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrdersRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuerySellOrdersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
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
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuerySellOrdersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.QuerySellOrdersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuerySellOrdersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuerySellOrdersRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrdersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuerySellOrdersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sell_orders.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrdersResponse", len)?;
        if !self.sell_orders.is_empty() {
            struct_ser.serialize_field("sellOrders", &self.sell_orders)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuerySellOrdersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sell_orders",
            "sellOrders",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SellOrders,
            Pagination,
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
                            "sellOrders" | "sell_orders" => Ok(GeneratedField::SellOrders),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuerySellOrdersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.QuerySellOrdersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuerySellOrdersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sell_orders__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SellOrders => {
                            if sell_orders__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sellOrders"));
                            }
                            sell_orders__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QuerySellOrdersResponse {
                    sell_orders: sell_orders__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.QuerySellOrdersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SellOrder {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.seller.is_empty() {
            len += 1;
        }
        if self.batch_key != 0 {
            len += 1;
        }
        if !self.quantity.is_empty() {
            len += 1;
        }
        if self.market_id != 0 {
            len += 1;
        }
        if !self.ask_amount.is_empty() {
            len += 1;
        }
        if self.disable_auto_retire {
            len += 1;
        }
        if self.expiration.is_some() {
            len += 1;
        }
        if self.maker {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.SellOrder", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if !self.seller.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("seller", pbjson::private::base64::encode(&self.seller).as_str())?;
        }
        if self.batch_key != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("batchKey", ToString::to_string(&self.batch_key).as_str())?;
        }
        if !self.quantity.is_empty() {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if self.market_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("marketId", ToString::to_string(&self.market_id).as_str())?;
        }
        if !self.ask_amount.is_empty() {
            struct_ser.serialize_field("askAmount", &self.ask_amount)?;
        }
        if self.disable_auto_retire {
            struct_ser.serialize_field("disableAutoRetire", &self.disable_auto_retire)?;
        }
        if let Some(v) = self.expiration.as_ref() {
            struct_ser.serialize_field("expiration", v)?;
        }
        if self.maker {
            struct_ser.serialize_field("maker", &self.maker)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SellOrder {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "seller",
            "batch_key",
            "batchKey",
            "quantity",
            "market_id",
            "marketId",
            "ask_amount",
            "askAmount",
            "disable_auto_retire",
            "disableAutoRetire",
            "expiration",
            "maker",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Seller,
            BatchKey,
            Quantity,
            MarketId,
            AskAmount,
            DisableAutoRetire,
            Expiration,
            Maker,
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
                            "id" => Ok(GeneratedField::Id),
                            "seller" => Ok(GeneratedField::Seller),
                            "batchKey" | "batch_key" => Ok(GeneratedField::BatchKey),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "marketId" | "market_id" => Ok(GeneratedField::MarketId),
                            "askAmount" | "ask_amount" => Ok(GeneratedField::AskAmount),
                            "disableAutoRetire" | "disable_auto_retire" => Ok(GeneratedField::DisableAutoRetire),
                            "expiration" => Ok(GeneratedField::Expiration),
                            "maker" => Ok(GeneratedField::Maker),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SellOrder;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.SellOrder")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SellOrder, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut seller__ = None;
                let mut batch_key__ = None;
                let mut quantity__ = None;
                let mut market_id__ = None;
                let mut ask_amount__ = None;
                let mut disable_auto_retire__ = None;
                let mut expiration__ = None;
                let mut maker__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Seller => {
                            if seller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seller"));
                            }
                            seller__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BatchKey => {
                            if batch_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchKey"));
                            }
                            batch_key__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::MarketId => {
                            if market_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("marketId"));
                            }
                            market_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AskAmount => {
                            if ask_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("askAmount"));
                            }
                            ask_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisableAutoRetire => {
                            if disable_auto_retire__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableAutoRetire"));
                            }
                            disable_auto_retire__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Expiration => {
                            if expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiration"));
                            }
                            expiration__ = map_.next_value()?;
                        }
                        GeneratedField::Maker => {
                            if maker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maker"));
                            }
                            maker__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(SellOrder {
                    id: id__.unwrap_or_default(),
                    seller: seller__.unwrap_or_default(),
                    batch_key: batch_key__.unwrap_or_default(),
                    quantity: quantity__.unwrap_or_default(),
                    market_id: market_id__.unwrap_or_default(),
                    ask_amount: ask_amount__.unwrap_or_default(),
                    disable_auto_retire: disable_auto_retire__.unwrap_or_default(),
                    expiration: expiration__,
                    maker: maker__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.SellOrder", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SellOrderInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.seller.is_empty() {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        if !self.quantity.is_empty() {
            len += 1;
        }
        if !self.ask_denom.is_empty() {
            len += 1;
        }
        if !self.ask_amount.is_empty() {
            len += 1;
        }
        if self.disable_auto_retire {
            len += 1;
        }
        if self.expiration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.marketplace.v1.SellOrderInfo", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if !self.seller.is_empty() {
            struct_ser.serialize_field("seller", &self.seller)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.quantity.is_empty() {
            struct_ser.serialize_field("quantity", &self.quantity)?;
        }
        if !self.ask_denom.is_empty() {
            struct_ser.serialize_field("askDenom", &self.ask_denom)?;
        }
        if !self.ask_amount.is_empty() {
            struct_ser.serialize_field("askAmount", &self.ask_amount)?;
        }
        if self.disable_auto_retire {
            struct_ser.serialize_field("disableAutoRetire", &self.disable_auto_retire)?;
        }
        if let Some(v) = self.expiration.as_ref() {
            struct_ser.serialize_field("expiration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SellOrderInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "seller",
            "batch_denom",
            "batchDenom",
            "quantity",
            "ask_denom",
            "askDenom",
            "ask_amount",
            "askAmount",
            "disable_auto_retire",
            "disableAutoRetire",
            "expiration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Seller,
            BatchDenom,
            Quantity,
            AskDenom,
            AskAmount,
            DisableAutoRetire,
            Expiration,
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
                            "id" => Ok(GeneratedField::Id),
                            "seller" => Ok(GeneratedField::Seller),
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "quantity" => Ok(GeneratedField::Quantity),
                            "askDenom" | "ask_denom" => Ok(GeneratedField::AskDenom),
                            "askAmount" | "ask_amount" => Ok(GeneratedField::AskAmount),
                            "disableAutoRetire" | "disable_auto_retire" => Ok(GeneratedField::DisableAutoRetire),
                            "expiration" => Ok(GeneratedField::Expiration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SellOrderInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.marketplace.v1.SellOrderInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SellOrderInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut seller__ = None;
                let mut batch_denom__ = None;
                let mut quantity__ = None;
                let mut ask_denom__ = None;
                let mut ask_amount__ = None;
                let mut disable_auto_retire__ = None;
                let mut expiration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Seller => {
                            if seller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seller"));
                            }
                            seller__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Quantity => {
                            if quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantity"));
                            }
                            quantity__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AskDenom => {
                            if ask_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("askDenom"));
                            }
                            ask_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AskAmount => {
                            if ask_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("askAmount"));
                            }
                            ask_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisableAutoRetire => {
                            if disable_auto_retire__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableAutoRetire"));
                            }
                            disable_auto_retire__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Expiration => {
                            if expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiration"));
                            }
                            expiration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SellOrderInfo {
                    id: id__.unwrap_or_default(),
                    seller: seller__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                    quantity: quantity__.unwrap_or_default(),
                    ask_denom: ask_denom__.unwrap_or_default(),
                    ask_amount: ask_amount__.unwrap_or_default(),
                    disable_auto_retire: disable_auto_retire__.unwrap_or_default(),
                    expiration: expiration__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.marketplace.v1.SellOrderInfo", FIELDS, GeneratedVisitor)
    }
}
