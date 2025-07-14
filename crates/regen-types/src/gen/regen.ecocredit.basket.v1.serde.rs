// @generated
impl serde::Serialize for Basket {
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
        if !self.basket_denom.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.disable_auto_retire {
            len += 1;
        }
        if !self.credit_type_abbrev.is_empty() {
            len += 1;
        }
        if self.date_criteria.is_some() {
            len += 1;
        }
        if self.exponent != 0 {
            len += 1;
        }
        if !self.curator.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.Basket", len)?;
        if self.id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if !self.basket_denom.is_empty() {
            struct_ser.serialize_field("basketDenom", &self.basket_denom)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.disable_auto_retire {
            struct_ser.serialize_field("disableAutoRetire", &self.disable_auto_retire)?;
        }
        if !self.credit_type_abbrev.is_empty() {
            struct_ser.serialize_field("creditTypeAbbrev", &self.credit_type_abbrev)?;
        }
        if let Some(v) = self.date_criteria.as_ref() {
            struct_ser.serialize_field("dateCriteria", v)?;
        }
        if self.exponent != 0 {
            struct_ser.serialize_field("exponent", &self.exponent)?;
        }
        if !self.curator.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("curator", pbjson::private::base64::encode(&self.curator).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Basket {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "basket_denom",
            "basketDenom",
            "name",
            "disable_auto_retire",
            "disableAutoRetire",
            "credit_type_abbrev",
            "creditTypeAbbrev",
            "date_criteria",
            "dateCriteria",
            "exponent",
            "curator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            BasketDenom,
            Name,
            DisableAutoRetire,
            CreditTypeAbbrev,
            DateCriteria,
            Exponent,
            Curator,
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
                            "basketDenom" | "basket_denom" => Ok(GeneratedField::BasketDenom),
                            "name" => Ok(GeneratedField::Name),
                            "disableAutoRetire" | "disable_auto_retire" => Ok(GeneratedField::DisableAutoRetire),
                            "creditTypeAbbrev" | "credit_type_abbrev" => Ok(GeneratedField::CreditTypeAbbrev),
                            "dateCriteria" | "date_criteria" => Ok(GeneratedField::DateCriteria),
                            "exponent" => Ok(GeneratedField::Exponent),
                            "curator" => Ok(GeneratedField::Curator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Basket;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.Basket")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Basket, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut basket_denom__ = None;
                let mut name__ = None;
                let mut disable_auto_retire__ = None;
                let mut credit_type_abbrev__ = None;
                let mut date_criteria__ = None;
                let mut exponent__ = None;
                let mut curator__ = None;
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
                        GeneratedField::BasketDenom => {
                            if basket_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketDenom"));
                            }
                            basket_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisableAutoRetire => {
                            if disable_auto_retire__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableAutoRetire"));
                            }
                            disable_auto_retire__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreditTypeAbbrev => {
                            if credit_type_abbrev__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditTypeAbbrev"));
                            }
                            credit_type_abbrev__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DateCriteria => {
                            if date_criteria__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateCriteria"));
                            }
                            date_criteria__ = map_.next_value()?;
                        }
                        GeneratedField::Exponent => {
                            if exponent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exponent"));
                            }
                            exponent__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Curator => {
                            if curator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("curator"));
                            }
                            curator__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Basket {
                    id: id__.unwrap_or_default(),
                    basket_denom: basket_denom__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    disable_auto_retire: disable_auto_retire__.unwrap_or_default(),
                    credit_type_abbrev: credit_type_abbrev__.unwrap_or_default(),
                    date_criteria: date_criteria__,
                    exponent: exponent__.unwrap_or_default(),
                    curator: curator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.Basket", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BasketBalance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.basket_id != 0 {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        if !self.balance.is_empty() {
            len += 1;
        }
        if self.batch_start_date.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.BasketBalance", len)?;
        if self.basket_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("basketId", ToString::to_string(&self.basket_id).as_str())?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.balance.is_empty() {
            struct_ser.serialize_field("balance", &self.balance)?;
        }
        if let Some(v) = self.batch_start_date.as_ref() {
            struct_ser.serialize_field("batchStartDate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BasketBalance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "basket_id",
            "basketId",
            "batch_denom",
            "batchDenom",
            "balance",
            "batch_start_date",
            "batchStartDate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BasketId,
            BatchDenom,
            Balance,
            BatchStartDate,
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
                            "basketId" | "basket_id" => Ok(GeneratedField::BasketId),
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "balance" => Ok(GeneratedField::Balance),
                            "batchStartDate" | "batch_start_date" => Ok(GeneratedField::BatchStartDate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BasketBalance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.BasketBalance")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BasketBalance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut basket_id__ = None;
                let mut batch_denom__ = None;
                let mut balance__ = None;
                let mut batch_start_date__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BasketId => {
                            if basket_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketId"));
                            }
                            basket_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BatchStartDate => {
                            if batch_start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchStartDate"));
                            }
                            batch_start_date__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BasketBalance {
                    basket_id: basket_id__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                    balance: balance__.unwrap_or_default(),
                    batch_start_date: batch_start_date__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.BasketBalance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BasketBalanceInfo {
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
        if !self.balance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.BasketBalanceInfo", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.balance.is_empty() {
            struct_ser.serialize_field("balance", &self.balance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BasketBalanceInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_denom",
            "batchDenom",
            "balance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchDenom,
            Balance,
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
                            "balance" => Ok(GeneratedField::Balance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BasketBalanceInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.BasketBalanceInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BasketBalanceInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                let mut balance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BasketBalanceInfo {
                    batch_denom: batch_denom__.unwrap_or_default(),
                    balance: balance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.BasketBalanceInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BasketClass {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.basket_id != 0 {
            len += 1;
        }
        if !self.class_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.BasketClass", len)?;
        if self.basket_id != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("basketId", ToString::to_string(&self.basket_id).as_str())?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BasketClass {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "basket_id",
            "basketId",
            "class_id",
            "classId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BasketId,
            ClassId,
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
                            "basketId" | "basket_id" => Ok(GeneratedField::BasketId),
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BasketClass;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.BasketClass")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BasketClass, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut basket_id__ = None;
                let mut class_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BasketId => {
                            if basket_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketId"));
                            }
                            basket_id__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BasketClass {
                    basket_id: basket_id__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.BasketClass", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BasketCredit {
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
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.BasketCredit", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BasketCredit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_denom",
            "batchDenom",
            "amount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchDenom,
            Amount,
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
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BasketCredit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.BasketCredit")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BasketCredit, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BasketCredit {
                    batch_denom: batch_denom__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.BasketCredit", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BasketFee {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fee.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.BasketFee", len)?;
        if let Some(v) = self.fee.as_ref() {
            struct_ser.serialize_field("fee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BasketFee {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fee,
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
                            "fee" => Ok(GeneratedField::Fee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BasketFee;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.BasketFee")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BasketFee, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fee__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BasketFee {
                    fee: fee__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.BasketFee", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BasketInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.basket_denom.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if self.disable_auto_retire {
            len += 1;
        }
        if !self.credit_type_abbrev.is_empty() {
            len += 1;
        }
        if self.date_criteria.is_some() {
            len += 1;
        }
        if self.exponent != 0 {
            len += 1;
        }
        if !self.curator.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.BasketInfo", len)?;
        if !self.basket_denom.is_empty() {
            struct_ser.serialize_field("basketDenom", &self.basket_denom)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if self.disable_auto_retire {
            struct_ser.serialize_field("disableAutoRetire", &self.disable_auto_retire)?;
        }
        if !self.credit_type_abbrev.is_empty() {
            struct_ser.serialize_field("creditTypeAbbrev", &self.credit_type_abbrev)?;
        }
        if let Some(v) = self.date_criteria.as_ref() {
            struct_ser.serialize_field("dateCriteria", v)?;
        }
        if self.exponent != 0 {
            struct_ser.serialize_field("exponent", &self.exponent)?;
        }
        if !self.curator.is_empty() {
            struct_ser.serialize_field("curator", &self.curator)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BasketInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "basket_denom",
            "basketDenom",
            "name",
            "disable_auto_retire",
            "disableAutoRetire",
            "credit_type_abbrev",
            "creditTypeAbbrev",
            "date_criteria",
            "dateCriteria",
            "exponent",
            "curator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BasketDenom,
            Name,
            DisableAutoRetire,
            CreditTypeAbbrev,
            DateCriteria,
            Exponent,
            Curator,
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
                            "basketDenom" | "basket_denom" => Ok(GeneratedField::BasketDenom),
                            "name" => Ok(GeneratedField::Name),
                            "disableAutoRetire" | "disable_auto_retire" => Ok(GeneratedField::DisableAutoRetire),
                            "creditTypeAbbrev" | "credit_type_abbrev" => Ok(GeneratedField::CreditTypeAbbrev),
                            "dateCriteria" | "date_criteria" => Ok(GeneratedField::DateCriteria),
                            "exponent" => Ok(GeneratedField::Exponent),
                            "curator" => Ok(GeneratedField::Curator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BasketInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.BasketInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BasketInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut basket_denom__ = None;
                let mut name__ = None;
                let mut disable_auto_retire__ = None;
                let mut credit_type_abbrev__ = None;
                let mut date_criteria__ = None;
                let mut exponent__ = None;
                let mut curator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BasketDenom => {
                            if basket_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketDenom"));
                            }
                            basket_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DisableAutoRetire => {
                            if disable_auto_retire__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableAutoRetire"));
                            }
                            disable_auto_retire__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreditTypeAbbrev => {
                            if credit_type_abbrev__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditTypeAbbrev"));
                            }
                            credit_type_abbrev__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DateCriteria => {
                            if date_criteria__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateCriteria"));
                            }
                            date_criteria__ = map_.next_value()?;
                        }
                        GeneratedField::Exponent => {
                            if exponent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exponent"));
                            }
                            exponent__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Curator => {
                            if curator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("curator"));
                            }
                            curator__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BasketInfo {
                    basket_denom: basket_denom__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    disable_auto_retire: disable_auto_retire__.unwrap_or_default(),
                    credit_type_abbrev: credit_type_abbrev__.unwrap_or_default(),
                    date_criteria: date_criteria__,
                    exponent: exponent__.unwrap_or_default(),
                    curator: curator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.BasketInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DateCriteria {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min_start_date.is_some() {
            len += 1;
        }
        if self.start_date_window.is_some() {
            len += 1;
        }
        if self.years_in_the_past != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.DateCriteria", len)?;
        if let Some(v) = self.min_start_date.as_ref() {
            struct_ser.serialize_field("minStartDate", v)?;
        }
        if let Some(v) = self.start_date_window.as_ref() {
            struct_ser.serialize_field("startDateWindow", v)?;
        }
        if self.years_in_the_past != 0 {
            struct_ser.serialize_field("yearsInThePast", &self.years_in_the_past)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DateCriteria {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min_start_date",
            "minStartDate",
            "start_date_window",
            "startDateWindow",
            "years_in_the_past",
            "yearsInThePast",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MinStartDate,
            StartDateWindow,
            YearsInThePast,
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
                            "minStartDate" | "min_start_date" => Ok(GeneratedField::MinStartDate),
                            "startDateWindow" | "start_date_window" => Ok(GeneratedField::StartDateWindow),
                            "yearsInThePast" | "years_in_the_past" => Ok(GeneratedField::YearsInThePast),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DateCriteria;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.DateCriteria")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DateCriteria, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min_start_date__ = None;
                let mut start_date_window__ = None;
                let mut years_in_the_past__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MinStartDate => {
                            if min_start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minStartDate"));
                            }
                            min_start_date__ = map_.next_value()?;
                        }
                        GeneratedField::StartDateWindow => {
                            if start_date_window__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startDateWindow"));
                            }
                            start_date_window__ = map_.next_value()?;
                        }
                        GeneratedField::YearsInThePast => {
                            if years_in_the_past__.is_some() {
                                return Err(serde::de::Error::duplicate_field("yearsInThePast"));
                            }
                            years_in_the_past__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DateCriteria {
                    min_start_date: min_start_date__,
                    start_date_window: start_date_window__,
                    years_in_the_past: years_in_the_past__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.DateCriteria", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventCreate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.basket_denom.is_empty() {
            len += 1;
        }
        if !self.curator.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.EventCreate", len)?;
        if !self.basket_denom.is_empty() {
            struct_ser.serialize_field("basketDenom", &self.basket_denom)?;
        }
        if !self.curator.is_empty() {
            struct_ser.serialize_field("curator", &self.curator)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventCreate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "basket_denom",
            "basketDenom",
            "curator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BasketDenom,
            Curator,
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
                            "basketDenom" | "basket_denom" => Ok(GeneratedField::BasketDenom),
                            "curator" => Ok(GeneratedField::Curator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventCreate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.EventCreate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventCreate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut basket_denom__ = None;
                let mut curator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BasketDenom => {
                            if basket_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketDenom"));
                            }
                            basket_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Curator => {
                            if curator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("curator"));
                            }
                            curator__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventCreate {
                    basket_denom: basket_denom__.unwrap_or_default(),
                    curator: curator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.EventCreate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventPut {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.basket_denom.is_empty() {
            len += 1;
        }
        if !self.credits.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.EventPut", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.basket_denom.is_empty() {
            struct_ser.serialize_field("basketDenom", &self.basket_denom)?;
        }
        if !self.credits.is_empty() {
            struct_ser.serialize_field("credits", &self.credits)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventPut {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "owner",
            "basket_denom",
            "basketDenom",
            "credits",
            "amount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            BasketDenom,
            Credits,
            Amount,
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
                            "owner" => Ok(GeneratedField::Owner),
                            "basketDenom" | "basket_denom" => Ok(GeneratedField::BasketDenom),
                            "credits" => Ok(GeneratedField::Credits),
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventPut;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.EventPut")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventPut, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut basket_denom__ = None;
                let mut credits__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BasketDenom => {
                            if basket_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketDenom"));
                            }
                            basket_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Credits => {
                            if credits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credits"));
                            }
                            credits__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventPut {
                    owner: owner__.unwrap_or_default(),
                    basket_denom: basket_denom__.unwrap_or_default(),
                    credits: credits__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.EventPut", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventTake {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.basket_denom.is_empty() {
            len += 1;
        }
        if !self.credits.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.EventTake", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.basket_denom.is_empty() {
            struct_ser.serialize_field("basketDenom", &self.basket_denom)?;
        }
        if !self.credits.is_empty() {
            struct_ser.serialize_field("credits", &self.credits)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventTake {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "owner",
            "basket_denom",
            "basketDenom",
            "credits",
            "amount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            BasketDenom,
            Credits,
            Amount,
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
                            "owner" => Ok(GeneratedField::Owner),
                            "basketDenom" | "basket_denom" => Ok(GeneratedField::BasketDenom),
                            "credits" => Ok(GeneratedField::Credits),
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventTake;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.EventTake")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventTake, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut basket_denom__ = None;
                let mut credits__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BasketDenom => {
                            if basket_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketDenom"));
                            }
                            basket_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Credits => {
                            if credits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credits"));
                            }
                            credits__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventTake {
                    owner: owner__.unwrap_or_default(),
                    basket_denom: basket_denom__.unwrap_or_default(),
                    credits: credits__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.EventTake", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventUpdateCurator {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.EventUpdateCurator", len)?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventUpdateCurator {
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
            type Value = EventUpdateCurator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.EventUpdateCurator")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventUpdateCurator, V::Error>
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
                Ok(EventUpdateCurator {
                    denom: denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.EventUpdateCurator", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventUpdateDateCriteria {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.EventUpdateDateCriteria", len)?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventUpdateDateCriteria {
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
            type Value = EventUpdateDateCriteria;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.EventUpdateDateCriteria")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventUpdateDateCriteria, V::Error>
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
                Ok(EventUpdateDateCriteria {
                    denom: denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.EventUpdateDateCriteria", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCreate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.curator.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.exponent != 0 {
            len += 1;
        }
        if self.disable_auto_retire {
            len += 1;
        }
        if !self.credit_type_abbrev.is_empty() {
            len += 1;
        }
        if !self.allowed_classes.is_empty() {
            len += 1;
        }
        if self.date_criteria.is_some() {
            len += 1;
        }
        if !self.fee.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.MsgCreate", len)?;
        if !self.curator.is_empty() {
            struct_ser.serialize_field("curator", &self.curator)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.exponent != 0 {
            struct_ser.serialize_field("exponent", &self.exponent)?;
        }
        if self.disable_auto_retire {
            struct_ser.serialize_field("disableAutoRetire", &self.disable_auto_retire)?;
        }
        if !self.credit_type_abbrev.is_empty() {
            struct_ser.serialize_field("creditTypeAbbrev", &self.credit_type_abbrev)?;
        }
        if !self.allowed_classes.is_empty() {
            struct_ser.serialize_field("allowedClasses", &self.allowed_classes)?;
        }
        if let Some(v) = self.date_criteria.as_ref() {
            struct_ser.serialize_field("dateCriteria", v)?;
        }
        if !self.fee.is_empty() {
            struct_ser.serialize_field("fee", &self.fee)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "curator",
            "name",
            "description",
            "exponent",
            "disable_auto_retire",
            "disableAutoRetire",
            "credit_type_abbrev",
            "creditTypeAbbrev",
            "allowed_classes",
            "allowedClasses",
            "date_criteria",
            "dateCriteria",
            "fee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Curator,
            Name,
            Description,
            Exponent,
            DisableAutoRetire,
            CreditTypeAbbrev,
            AllowedClasses,
            DateCriteria,
            Fee,
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
                            "curator" => Ok(GeneratedField::Curator),
                            "name" => Ok(GeneratedField::Name),
                            "description" => Ok(GeneratedField::Description),
                            "exponent" => Ok(GeneratedField::Exponent),
                            "disableAutoRetire" | "disable_auto_retire" => Ok(GeneratedField::DisableAutoRetire),
                            "creditTypeAbbrev" | "credit_type_abbrev" => Ok(GeneratedField::CreditTypeAbbrev),
                            "allowedClasses" | "allowed_classes" => Ok(GeneratedField::AllowedClasses),
                            "dateCriteria" | "date_criteria" => Ok(GeneratedField::DateCriteria),
                            "fee" => Ok(GeneratedField::Fee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.MsgCreate")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgCreate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut curator__ = None;
                let mut name__ = None;
                let mut description__ = None;
                let mut exponent__ = None;
                let mut disable_auto_retire__ = None;
                let mut credit_type_abbrev__ = None;
                let mut allowed_classes__ = None;
                let mut date_criteria__ = None;
                let mut fee__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Curator => {
                            if curator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("curator"));
                            }
                            curator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Exponent => {
                            if exponent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exponent"));
                            }
                            exponent__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DisableAutoRetire => {
                            if disable_auto_retire__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disableAutoRetire"));
                            }
                            disable_auto_retire__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreditTypeAbbrev => {
                            if credit_type_abbrev__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditTypeAbbrev"));
                            }
                            credit_type_abbrev__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowedClasses => {
                            if allowed_classes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedClasses"));
                            }
                            allowed_classes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DateCriteria => {
                            if date_criteria__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dateCriteria"));
                            }
                            date_criteria__ = map_.next_value()?;
                        }
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgCreate {
                    curator: curator__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    exponent: exponent__.unwrap_or_default(),
                    disable_auto_retire: disable_auto_retire__.unwrap_or_default(),
                    credit_type_abbrev: credit_type_abbrev__.unwrap_or_default(),
                    allowed_classes: allowed_classes__.unwrap_or_default(),
                    date_criteria: date_criteria__,
                    fee: fee__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.MsgCreate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCreateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.basket_denom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.MsgCreateResponse", len)?;
        if !self.basket_denom.is_empty() {
            struct_ser.serialize_field("basketDenom", &self.basket_denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "basket_denom",
            "basketDenom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BasketDenom,
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
                            "basketDenom" | "basket_denom" => Ok(GeneratedField::BasketDenom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.MsgCreateResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgCreateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut basket_denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BasketDenom => {
                            if basket_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketDenom"));
                            }
                            basket_denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgCreateResponse {
                    basket_denom: basket_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.MsgCreateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgPut {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.basket_denom.is_empty() {
            len += 1;
        }
        if !self.credits.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.MsgPut", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.basket_denom.is_empty() {
            struct_ser.serialize_field("basketDenom", &self.basket_denom)?;
        }
        if !self.credits.is_empty() {
            struct_ser.serialize_field("credits", &self.credits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgPut {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "owner",
            "basket_denom",
            "basketDenom",
            "credits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            BasketDenom,
            Credits,
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
                            "owner" => Ok(GeneratedField::Owner),
                            "basketDenom" | "basket_denom" => Ok(GeneratedField::BasketDenom),
                            "credits" => Ok(GeneratedField::Credits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgPut;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.MsgPut")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgPut, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut basket_denom__ = None;
                let mut credits__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BasketDenom => {
                            if basket_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketDenom"));
                            }
                            basket_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Credits => {
                            if credits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credits"));
                            }
                            credits__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgPut {
                    owner: owner__.unwrap_or_default(),
                    basket_denom: basket_denom__.unwrap_or_default(),
                    credits: credits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.MsgPut", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgPutResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.amount_received.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.MsgPutResponse", len)?;
        if !self.amount_received.is_empty() {
            struct_ser.serialize_field("amountReceived", &self.amount_received)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgPutResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "amount_received",
            "amountReceived",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AmountReceived,
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
                            "amountReceived" | "amount_received" => Ok(GeneratedField::AmountReceived),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgPutResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.MsgPutResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgPutResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut amount_received__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::AmountReceived => {
                            if amount_received__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amountReceived"));
                            }
                            amount_received__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgPutResponse {
                    amount_received: amount_received__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.MsgPutResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgTake {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.basket_denom.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.retirement_location.is_empty() {
            len += 1;
        }
        if self.retire_on_take {
            len += 1;
        }
        if !self.retirement_jurisdiction.is_empty() {
            len += 1;
        }
        if !self.retirement_reason.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.MsgTake", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.basket_denom.is_empty() {
            struct_ser.serialize_field("basketDenom", &self.basket_denom)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.retirement_location.is_empty() {
            struct_ser.serialize_field("retirementLocation", &self.retirement_location)?;
        }
        if self.retire_on_take {
            struct_ser.serialize_field("retireOnTake", &self.retire_on_take)?;
        }
        if !self.retirement_jurisdiction.is_empty() {
            struct_ser.serialize_field("retirementJurisdiction", &self.retirement_jurisdiction)?;
        }
        if !self.retirement_reason.is_empty() {
            struct_ser.serialize_field("retirementReason", &self.retirement_reason)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgTake {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "owner",
            "basket_denom",
            "basketDenom",
            "amount",
            "retirement_location",
            "retirementLocation",
            "retire_on_take",
            "retireOnTake",
            "retirement_jurisdiction",
            "retirementJurisdiction",
            "retirement_reason",
            "retirementReason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            BasketDenom,
            Amount,
            RetirementLocation,
            RetireOnTake,
            RetirementJurisdiction,
            RetirementReason,
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
                            "owner" => Ok(GeneratedField::Owner),
                            "basketDenom" | "basket_denom" => Ok(GeneratedField::BasketDenom),
                            "amount" => Ok(GeneratedField::Amount),
                            "retirementLocation" | "retirement_location" => Ok(GeneratedField::RetirementLocation),
                            "retireOnTake" | "retire_on_take" => Ok(GeneratedField::RetireOnTake),
                            "retirementJurisdiction" | "retirement_jurisdiction" => Ok(GeneratedField::RetirementJurisdiction),
                            "retirementReason" | "retirement_reason" => Ok(GeneratedField::RetirementReason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgTake;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.MsgTake")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgTake, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut basket_denom__ = None;
                let mut amount__ = None;
                let mut retirement_location__ = None;
                let mut retire_on_take__ = None;
                let mut retirement_jurisdiction__ = None;
                let mut retirement_reason__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BasketDenom => {
                            if basket_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketDenom"));
                            }
                            basket_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetirementLocation => {
                            if retirement_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retirementLocation"));
                            }
                            retirement_location__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetireOnTake => {
                            if retire_on_take__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retireOnTake"));
                            }
                            retire_on_take__ = Some(map_.next_value()?);
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
                    }
                }
                Ok(MsgTake {
                    owner: owner__.unwrap_or_default(),
                    basket_denom: basket_denom__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    retirement_location: retirement_location__.unwrap_or_default(),
                    retire_on_take: retire_on_take__.unwrap_or_default(),
                    retirement_jurisdiction: retirement_jurisdiction__.unwrap_or_default(),
                    retirement_reason: retirement_reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.MsgTake", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgTakeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.credits.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.MsgTakeResponse", len)?;
        if !self.credits.is_empty() {
            struct_ser.serialize_field("credits", &self.credits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgTakeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "credits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Credits,
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
                            "credits" => Ok(GeneratedField::Credits),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgTakeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.MsgTakeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgTakeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credits__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Credits => {
                            if credits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credits"));
                            }
                            credits__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgTakeResponse {
                    credits: credits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.MsgTakeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateBasketFee {
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
        if self.fee.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.MsgUpdateBasketFee", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.fee.as_ref() {
            struct_ser.serialize_field("fee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateBasketFee {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "fee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Fee,
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
                            "fee" => Ok(GeneratedField::Fee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateBasketFee;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.MsgUpdateBasketFee")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateBasketFee, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut fee__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgUpdateBasketFee {
                    authority: authority__.unwrap_or_default(),
                    fee: fee__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.MsgUpdateBasketFee", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateBasketFeeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.MsgUpdateBasketFeeResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateBasketFeeResponse {
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
            type Value = MsgUpdateBasketFeeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.MsgUpdateBasketFeeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateBasketFeeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateBasketFeeResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.MsgUpdateBasketFeeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateCurator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.curator.is_empty() {
            len += 1;
        }
        if !self.denom.is_empty() {
            len += 1;
        }
        if !self.new_curator.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.MsgUpdateCurator", len)?;
        if !self.curator.is_empty() {
            struct_ser.serialize_field("curator", &self.curator)?;
        }
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if !self.new_curator.is_empty() {
            struct_ser.serialize_field("newCurator", &self.new_curator)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateCurator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "curator",
            "denom",
            "new_curator",
            "newCurator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Curator,
            Denom,
            NewCurator,
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
                            "curator" => Ok(GeneratedField::Curator),
                            "denom" => Ok(GeneratedField::Denom),
                            "newCurator" | "new_curator" => Ok(GeneratedField::NewCurator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateCurator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.MsgUpdateCurator")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateCurator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut curator__ = None;
                let mut denom__ = None;
                let mut new_curator__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Curator => {
                            if curator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("curator"));
                            }
                            curator__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewCurator => {
                            if new_curator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newCurator"));
                            }
                            new_curator__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateCurator {
                    curator: curator__.unwrap_or_default(),
                    denom: denom__.unwrap_or_default(),
                    new_curator: new_curator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.MsgUpdateCurator", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateCuratorResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.MsgUpdateCuratorResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateCuratorResponse {
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
            type Value = MsgUpdateCuratorResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.MsgUpdateCuratorResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateCuratorResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateCuratorResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.MsgUpdateCuratorResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateDateCriteria {
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
        if self.new_date_criteria.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.MsgUpdateDateCriteria", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if let Some(v) = self.new_date_criteria.as_ref() {
            struct_ser.serialize_field("newDateCriteria", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateDateCriteria {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "denom",
            "new_date_criteria",
            "newDateCriteria",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Denom,
            NewDateCriteria,
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
                            "newDateCriteria" | "new_date_criteria" => Ok(GeneratedField::NewDateCriteria),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateDateCriteria;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.MsgUpdateDateCriteria")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateDateCriteria, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut denom__ = None;
                let mut new_date_criteria__ = None;
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
                        GeneratedField::NewDateCriteria => {
                            if new_date_criteria__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newDateCriteria"));
                            }
                            new_date_criteria__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MsgUpdateDateCriteria {
                    authority: authority__.unwrap_or_default(),
                    denom: denom__.unwrap_or_default(),
                    new_date_criteria: new_date_criteria__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.MsgUpdateDateCriteria", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateDateCriteriaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.MsgUpdateDateCriteriaResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateDateCriteriaResponse {
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
            type Value = MsgUpdateDateCriteriaResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.MsgUpdateDateCriteriaResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateDateCriteriaResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateDateCriteriaResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.MsgUpdateDateCriteriaResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBasketBalanceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.basket_denom.is_empty() {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.QueryBasketBalanceRequest", len)?;
        if !self.basket_denom.is_empty() {
            struct_ser.serialize_field("basketDenom", &self.basket_denom)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBasketBalanceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "basket_denom",
            "basketDenom",
            "batch_denom",
            "batchDenom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BasketDenom,
            BatchDenom,
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
                            "basketDenom" | "basket_denom" => Ok(GeneratedField::BasketDenom),
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBasketBalanceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.QueryBasketBalanceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBasketBalanceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut basket_denom__ = None;
                let mut batch_denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BasketDenom => {
                            if basket_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketDenom"));
                            }
                            basket_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBasketBalanceRequest {
                    basket_denom: basket_denom__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.QueryBasketBalanceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBasketBalanceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.balance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.QueryBasketBalanceResponse", len)?;
        if !self.balance.is_empty() {
            struct_ser.serialize_field("balance", &self.balance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBasketBalanceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "balance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Balance,
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
                            "balance" => Ok(GeneratedField::Balance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBasketBalanceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.QueryBasketBalanceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBasketBalanceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut balance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBasketBalanceResponse {
                    balance: balance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.QueryBasketBalanceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBasketBalancesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.basket_denom.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.QueryBasketBalancesRequest", len)?;
        if !self.basket_denom.is_empty() {
            struct_ser.serialize_field("basketDenom", &self.basket_denom)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBasketBalancesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "basket_denom",
            "basketDenom",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BasketDenom,
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
                            "basketDenom" | "basket_denom" => Ok(GeneratedField::BasketDenom),
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
            type Value = QueryBasketBalancesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.QueryBasketBalancesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBasketBalancesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut basket_denom__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BasketDenom => {
                            if basket_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketDenom"));
                            }
                            basket_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryBasketBalancesRequest {
                    basket_denom: basket_denom__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.QueryBasketBalancesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBasketBalancesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.balances.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        if !self.balances_info.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.QueryBasketBalancesResponse", len)?;
        if !self.balances.is_empty() {
            struct_ser.serialize_field("balances", &self.balances)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if !self.balances_info.is_empty() {
            struct_ser.serialize_field("balancesInfo", &self.balances_info)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBasketBalancesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "balances",
            "pagination",
            "balances_info",
            "balancesInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Balances,
            Pagination,
            BalancesInfo,
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
                            "balances" => Ok(GeneratedField::Balances),
                            "pagination" => Ok(GeneratedField::Pagination),
                            "balancesInfo" | "balances_info" => Ok(GeneratedField::BalancesInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBasketBalancesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.QueryBasketBalancesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBasketBalancesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut balances__ = None;
                let mut pagination__ = None;
                let mut balances_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Balances => {
                            if balances__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balances"));
                            }
                            balances__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::BalancesInfo => {
                            if balances_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balancesInfo"));
                            }
                            balances_info__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBasketBalancesResponse {
                    balances: balances__.unwrap_or_default(),
                    pagination: pagination__,
                    balances_info: balances_info__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.QueryBasketBalancesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBasketFeeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.QueryBasketFeeRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBasketFeeRequest {
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
            type Value = QueryBasketFeeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.QueryBasketFeeRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBasketFeeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryBasketFeeRequest {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.QueryBasketFeeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBasketFeeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.fee.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.QueryBasketFeeResponse", len)?;
        if let Some(v) = self.fee.as_ref() {
            struct_ser.serialize_field("fee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBasketFeeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "fee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Fee,
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
                            "fee" => Ok(GeneratedField::Fee),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBasketFeeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.QueryBasketFeeResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBasketFeeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fee__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryBasketFeeResponse {
                    fee: fee__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.QueryBasketFeeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBasketRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.basket_denom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.QueryBasketRequest", len)?;
        if !self.basket_denom.is_empty() {
            struct_ser.serialize_field("basketDenom", &self.basket_denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBasketRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "basket_denom",
            "basketDenom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BasketDenom,
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
                            "basketDenom" | "basket_denom" => Ok(GeneratedField::BasketDenom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBasketRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.QueryBasketRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBasketRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut basket_denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BasketDenom => {
                            if basket_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketDenom"));
                            }
                            basket_denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBasketRequest {
                    basket_denom: basket_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.QueryBasketRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBasketResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.basket.is_some() {
            len += 1;
        }
        if !self.classes.is_empty() {
            len += 1;
        }
        if self.basket_info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.QueryBasketResponse", len)?;
        if let Some(v) = self.basket.as_ref() {
            struct_ser.serialize_field("basket", v)?;
        }
        if !self.classes.is_empty() {
            struct_ser.serialize_field("classes", &self.classes)?;
        }
        if let Some(v) = self.basket_info.as_ref() {
            struct_ser.serialize_field("basketInfo", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBasketResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "basket",
            "classes",
            "basket_info",
            "basketInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Basket,
            Classes,
            BasketInfo,
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
                            "basket" => Ok(GeneratedField::Basket),
                            "classes" => Ok(GeneratedField::Classes),
                            "basketInfo" | "basket_info" => Ok(GeneratedField::BasketInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBasketResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.QueryBasketResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBasketResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut basket__ = None;
                let mut classes__ = None;
                let mut basket_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Basket => {
                            if basket__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basket"));
                            }
                            basket__ = map_.next_value()?;
                        }
                        GeneratedField::Classes => {
                            if classes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classes"));
                            }
                            classes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BasketInfo => {
                            if basket_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketInfo"));
                            }
                            basket_info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryBasketResponse {
                    basket: basket__,
                    classes: classes__.unwrap_or_default(),
                    basket_info: basket_info__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.QueryBasketResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBasketsRequest {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.QueryBasketsRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBasketsRequest {
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
            type Value = QueryBasketsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.QueryBasketsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBasketsRequest, V::Error>
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
                Ok(QueryBasketsRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.QueryBasketsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBasketsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.baskets.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        if !self.baskets_info.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.basket.v1.QueryBasketsResponse", len)?;
        if !self.baskets.is_empty() {
            struct_ser.serialize_field("baskets", &self.baskets)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if !self.baskets_info.is_empty() {
            struct_ser.serialize_field("basketsInfo", &self.baskets_info)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBasketsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "baskets",
            "pagination",
            "baskets_info",
            "basketsInfo",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Baskets,
            Pagination,
            BasketsInfo,
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
                            "baskets" => Ok(GeneratedField::Baskets),
                            "pagination" => Ok(GeneratedField::Pagination),
                            "basketsInfo" | "baskets_info" => Ok(GeneratedField::BasketsInfo),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBasketsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.basket.v1.QueryBasketsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBasketsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut baskets__ = None;
                let mut pagination__ = None;
                let mut baskets_info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Baskets => {
                            if baskets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baskets"));
                            }
                            baskets__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                        GeneratedField::BasketsInfo => {
                            if baskets_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketsInfo"));
                            }
                            baskets_info__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBasketsResponse {
                    baskets: baskets__.unwrap_or_default(),
                    pagination: pagination__,
                    baskets_info: baskets_info__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.basket.v1.QueryBasketsResponse", FIELDS, GeneratedVisitor)
    }
}
