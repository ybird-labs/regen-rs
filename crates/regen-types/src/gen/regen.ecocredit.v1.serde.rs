// @generated
impl serde::Serialize for AllowedBridgeChain {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.AllowedBridgeChain", len)?;
        if !self.chain_name.is_empty() {
            struct_ser.serialize_field("chainName", &self.chain_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllowedBridgeChain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_name",
            "chainName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainName,
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
                            "chainName" | "chain_name" => Ok(GeneratedField::ChainName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AllowedBridgeChain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.AllowedBridgeChain")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AllowedBridgeChain, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut chain_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainName => {
                            if chain_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainName"));
                            }
                            chain_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AllowedBridgeChain {
                    chain_name: chain_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.AllowedBridgeChain", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AllowedClassCreator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.AllowedClassCreator", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", pbjson::private::base64::encode(&self.address).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllowedClassCreator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AllowedClassCreator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.AllowedClassCreator")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AllowedClassCreator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(AllowedClassCreator {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.AllowedClassCreator", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.AllowedDenom", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1.AllowedDenom")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AllowedDenom, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bank_denom__ = None;
                let mut display_denom__ = None;
                let mut exponent__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BankDenom => {
                            if bank_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bankDenom"));
                            }
                            bank_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::DisplayDenom => {
                            if display_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayDenom"));
                            }
                            display_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::Exponent => {
                            if exponent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exponent"));
                            }
                            exponent__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
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
        deserializer.deserialize_struct("regen.ecocredit.v1.AllowedDenom", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Batch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key != 0 {
            len += 1;
        }
        if !self.issuer.is_empty() {
            len += 1;
        }
        if self.project_key != 0 {
            len += 1;
        }
        if !self.denom.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.start_date.is_some() {
            len += 1;
        }
        if self.end_date.is_some() {
            len += 1;
        }
        if self.issuance_date.is_some() {
            len += 1;
        }
        if self.open {
            len += 1;
        }
        if self.class_key != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.Batch", len)?;
        if self.key != 0 {
            struct_ser.serialize_field("key", ToString::to_string(&self.key).as_str())?;
        }
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", pbjson::private::base64::encode(&self.issuer).as_str())?;
        }
        if self.project_key != 0 {
            struct_ser.serialize_field("projectKey", ToString::to_string(&self.project_key).as_str())?;
        }
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if let Some(v) = self.start_date.as_ref() {
            struct_ser.serialize_field("startDate", v)?;
        }
        if let Some(v) = self.end_date.as_ref() {
            struct_ser.serialize_field("endDate", v)?;
        }
        if let Some(v) = self.issuance_date.as_ref() {
            struct_ser.serialize_field("issuanceDate", v)?;
        }
        if self.open {
            struct_ser.serialize_field("open", &self.open)?;
        }
        if self.class_key != 0 {
            struct_ser.serialize_field("classKey", ToString::to_string(&self.class_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Batch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "issuer",
            "project_key",
            "projectKey",
            "denom",
            "metadata",
            "start_date",
            "startDate",
            "end_date",
            "endDate",
            "issuance_date",
            "issuanceDate",
            "open",
            "class_key",
            "classKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Issuer,
            ProjectKey,
            Denom,
            Metadata,
            StartDate,
            EndDate,
            IssuanceDate,
            Open,
            ClassKey,
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
                            "key" => Ok(GeneratedField::Key),
                            "issuer" => Ok(GeneratedField::Issuer),
                            "projectKey" | "project_key" => Ok(GeneratedField::ProjectKey),
                            "denom" => Ok(GeneratedField::Denom),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "startDate" | "start_date" => Ok(GeneratedField::StartDate),
                            "endDate" | "end_date" => Ok(GeneratedField::EndDate),
                            "issuanceDate" | "issuance_date" => Ok(GeneratedField::IssuanceDate),
                            "open" => Ok(GeneratedField::Open),
                            "classKey" | "class_key" => Ok(GeneratedField::ClassKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Batch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.Batch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Batch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut issuer__ = None;
                let mut project_key__ = None;
                let mut denom__ = None;
                let mut metadata__ = None;
                let mut start_date__ = None;
                let mut end_date__ = None;
                let mut issuance_date__ = None;
                let mut open__ = None;
                let mut class_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProjectKey => {
                            if project_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectKey"));
                            }
                            project_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::StartDate => {
                            if start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startDate"));
                            }
                            start_date__ = map.next_value()?;
                        }
                        GeneratedField::EndDate => {
                            if end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endDate"));
                            }
                            end_date__ = map.next_value()?;
                        }
                        GeneratedField::IssuanceDate => {
                            if issuance_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuanceDate"));
                            }
                            issuance_date__ = map.next_value()?;
                        }
                        GeneratedField::Open => {
                            if open__.is_some() {
                                return Err(serde::de::Error::duplicate_field("open"));
                            }
                            open__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClassKey => {
                            if class_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classKey"));
                            }
                            class_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Batch {
                    key: key__.unwrap_or_default(),
                    issuer: issuer__.unwrap_or_default(),
                    project_key: project_key__.unwrap_or_default(),
                    denom: denom__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    start_date: start_date__,
                    end_date: end_date__,
                    issuance_date: issuance_date__,
                    open: open__.unwrap_or_default(),
                    class_key: class_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.Batch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchBalance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.batch_key != 0 {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.tradable_amount.is_empty() {
            len += 1;
        }
        if !self.retired_amount.is_empty() {
            len += 1;
        }
        if !self.escrowed_amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.BatchBalance", len)?;
        if self.batch_key != 0 {
            struct_ser.serialize_field("batchKey", ToString::to_string(&self.batch_key).as_str())?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", pbjson::private::base64::encode(&self.address).as_str())?;
        }
        if !self.tradable_amount.is_empty() {
            struct_ser.serialize_field("tradableAmount", &self.tradable_amount)?;
        }
        if !self.retired_amount.is_empty() {
            struct_ser.serialize_field("retiredAmount", &self.retired_amount)?;
        }
        if !self.escrowed_amount.is_empty() {
            struct_ser.serialize_field("escrowedAmount", &self.escrowed_amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchBalance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_key",
            "batchKey",
            "address",
            "tradable_amount",
            "tradableAmount",
            "retired_amount",
            "retiredAmount",
            "escrowed_amount",
            "escrowedAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchKey,
            Address,
            TradableAmount,
            RetiredAmount,
            EscrowedAmount,
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
                            "batchKey" | "batch_key" => Ok(GeneratedField::BatchKey),
                            "address" => Ok(GeneratedField::Address),
                            "tradableAmount" | "tradable_amount" => Ok(GeneratedField::TradableAmount),
                            "retiredAmount" | "retired_amount" => Ok(GeneratedField::RetiredAmount),
                            "escrowedAmount" | "escrowed_amount" => Ok(GeneratedField::EscrowedAmount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchBalance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.BatchBalance")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BatchBalance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_key__ = None;
                let mut address__ = None;
                let mut tradable_amount__ = None;
                let mut retired_amount__ = None;
                let mut escrowed_amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BatchKey => {
                            if batch_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchKey"));
                            }
                            batch_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TradableAmount => {
                            if tradable_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradableAmount"));
                            }
                            tradable_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetiredAmount => {
                            if retired_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retiredAmount"));
                            }
                            retired_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::EscrowedAmount => {
                            if escrowed_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("escrowedAmount"));
                            }
                            escrowed_amount__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BatchBalance {
                    batch_key: batch_key__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    tradable_amount: tradable_amount__.unwrap_or_default(),
                    retired_amount: retired_amount__.unwrap_or_default(),
                    escrowed_amount: escrowed_amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.BatchBalance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchBalanceInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        if !self.tradable_amount.is_empty() {
            len += 1;
        }
        if !self.retired_amount.is_empty() {
            len += 1;
        }
        if !self.escrowed_amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.BatchBalanceInfo", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.tradable_amount.is_empty() {
            struct_ser.serialize_field("tradableAmount", &self.tradable_amount)?;
        }
        if !self.retired_amount.is_empty() {
            struct_ser.serialize_field("retiredAmount", &self.retired_amount)?;
        }
        if !self.escrowed_amount.is_empty() {
            struct_ser.serialize_field("escrowedAmount", &self.escrowed_amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchBalanceInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "batch_denom",
            "batchDenom",
            "tradable_amount",
            "tradableAmount",
            "retired_amount",
            "retiredAmount",
            "escrowed_amount",
            "escrowedAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            BatchDenom,
            TradableAmount,
            RetiredAmount,
            EscrowedAmount,
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
                            "address" => Ok(GeneratedField::Address),
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "tradableAmount" | "tradable_amount" => Ok(GeneratedField::TradableAmount),
                            "retiredAmount" | "retired_amount" => Ok(GeneratedField::RetiredAmount),
                            "escrowedAmount" | "escrowed_amount" => Ok(GeneratedField::EscrowedAmount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchBalanceInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.BatchBalanceInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BatchBalanceInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut batch_denom__ = None;
                let mut tradable_amount__ = None;
                let mut retired_amount__ = None;
                let mut escrowed_amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::TradableAmount => {
                            if tradable_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradableAmount"));
                            }
                            tradable_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetiredAmount => {
                            if retired_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retiredAmount"));
                            }
                            retired_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::EscrowedAmount => {
                            if escrowed_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("escrowedAmount"));
                            }
                            escrowed_amount__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BatchBalanceInfo {
                    address: address__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                    tradable_amount: tradable_amount__.unwrap_or_default(),
                    retired_amount: retired_amount__.unwrap_or_default(),
                    escrowed_amount: escrowed_amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.BatchBalanceInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchContract {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.batch_key != 0 {
            len += 1;
        }
        if self.class_key != 0 {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.BatchContract", len)?;
        if self.batch_key != 0 {
            struct_ser.serialize_field("batchKey", ToString::to_string(&self.batch_key).as_str())?;
        }
        if self.class_key != 0 {
            struct_ser.serialize_field("classKey", ToString::to_string(&self.class_key).as_str())?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchContract {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_key",
            "batchKey",
            "class_key",
            "classKey",
            "contract",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchKey,
            ClassKey,
            Contract,
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
                            "batchKey" | "batch_key" => Ok(GeneratedField::BatchKey),
                            "classKey" | "class_key" => Ok(GeneratedField::ClassKey),
                            "contract" => Ok(GeneratedField::Contract),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchContract;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.BatchContract")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BatchContract, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_key__ = None;
                let mut class_key__ = None;
                let mut contract__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BatchKey => {
                            if batch_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchKey"));
                            }
                            batch_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ClassKey => {
                            if class_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classKey"));
                            }
                            class_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BatchContract {
                    batch_key: batch_key__.unwrap_or_default(),
                    class_key: class_key__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.BatchContract", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.denom.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.start_date.is_some() {
            len += 1;
        }
        if self.end_date.is_some() {
            len += 1;
        }
        if self.issuance_date.is_some() {
            len += 1;
        }
        if self.open {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.BatchInfo", len)?;
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if let Some(v) = self.start_date.as_ref() {
            struct_ser.serialize_field("startDate", v)?;
        }
        if let Some(v) = self.end_date.as_ref() {
            struct_ser.serialize_field("endDate", v)?;
        }
        if let Some(v) = self.issuance_date.as_ref() {
            struct_ser.serialize_field("issuanceDate", v)?;
        }
        if self.open {
            struct_ser.serialize_field("open", &self.open)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "issuer",
            "project_id",
            "projectId",
            "denom",
            "metadata",
            "start_date",
            "startDate",
            "end_date",
            "endDate",
            "issuance_date",
            "issuanceDate",
            "open",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Issuer,
            ProjectId,
            Denom,
            Metadata,
            StartDate,
            EndDate,
            IssuanceDate,
            Open,
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
                            "issuer" => Ok(GeneratedField::Issuer),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "denom" => Ok(GeneratedField::Denom),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "startDate" | "start_date" => Ok(GeneratedField::StartDate),
                            "endDate" | "end_date" => Ok(GeneratedField::EndDate),
                            "issuanceDate" | "issuance_date" => Ok(GeneratedField::IssuanceDate),
                            "open" => Ok(GeneratedField::Open),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.BatchInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BatchInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut issuer__ = None;
                let mut project_id__ = None;
                let mut denom__ = None;
                let mut metadata__ = None;
                let mut start_date__ = None;
                let mut end_date__ = None;
                let mut issuance_date__ = None;
                let mut open__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::StartDate => {
                            if start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startDate"));
                            }
                            start_date__ = map.next_value()?;
                        }
                        GeneratedField::EndDate => {
                            if end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endDate"));
                            }
                            end_date__ = map.next_value()?;
                        }
                        GeneratedField::IssuanceDate => {
                            if issuance_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuanceDate"));
                            }
                            issuance_date__ = map.next_value()?;
                        }
                        GeneratedField::Open => {
                            if open__.is_some() {
                                return Err(serde::de::Error::duplicate_field("open"));
                            }
                            open__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BatchInfo {
                    issuer: issuer__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    denom: denom__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    start_date: start_date__,
                    end_date: end_date__,
                    issuance_date: issuance_date__,
                    open: open__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.BatchInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchIssuance {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.tradable_amount.is_empty() {
            len += 1;
        }
        if !self.retired_amount.is_empty() {
            len += 1;
        }
        if !self.retirement_jurisdiction.is_empty() {
            len += 1;
        }
        if !self.retirement_reason.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.BatchIssuance", len)?;
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.tradable_amount.is_empty() {
            struct_ser.serialize_field("tradableAmount", &self.tradable_amount)?;
        }
        if !self.retired_amount.is_empty() {
            struct_ser.serialize_field("retiredAmount", &self.retired_amount)?;
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
impl<'de> serde::Deserialize<'de> for BatchIssuance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "recipient",
            "tradable_amount",
            "tradableAmount",
            "retired_amount",
            "retiredAmount",
            "retirement_jurisdiction",
            "retirementJurisdiction",
            "retirement_reason",
            "retirementReason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Recipient,
            TradableAmount,
            RetiredAmount,
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
                            "recipient" => Ok(GeneratedField::Recipient),
                            "tradableAmount" | "tradable_amount" => Ok(GeneratedField::TradableAmount),
                            "retiredAmount" | "retired_amount" => Ok(GeneratedField::RetiredAmount),
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
            type Value = BatchIssuance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.BatchIssuance")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BatchIssuance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut recipient__ = None;
                let mut tradable_amount__ = None;
                let mut retired_amount__ = None;
                let mut retirement_jurisdiction__ = None;
                let mut retirement_reason__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map.next_value()?);
                        }
                        GeneratedField::TradableAmount => {
                            if tradable_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradableAmount"));
                            }
                            tradable_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetiredAmount => {
                            if retired_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retiredAmount"));
                            }
                            retired_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetirementJurisdiction => {
                            if retirement_jurisdiction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retirementJurisdiction"));
                            }
                            retirement_jurisdiction__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetirementReason => {
                            if retirement_reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retirementReason"));
                            }
                            retirement_reason__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BatchIssuance {
                    recipient: recipient__.unwrap_or_default(),
                    tradable_amount: tradable_amount__.unwrap_or_default(),
                    retired_amount: retired_amount__.unwrap_or_default(),
                    retirement_jurisdiction: retirement_jurisdiction__.unwrap_or_default(),
                    retirement_reason: retirement_reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.BatchIssuance", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchSequence {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.project_key != 0 {
            len += 1;
        }
        if self.next_sequence != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.BatchSequence", len)?;
        if self.project_key != 0 {
            struct_ser.serialize_field("projectKey", ToString::to_string(&self.project_key).as_str())?;
        }
        if self.next_sequence != 0 {
            struct_ser.serialize_field("nextSequence", ToString::to_string(&self.next_sequence).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchSequence {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_key",
            "projectKey",
            "next_sequence",
            "nextSequence",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectKey,
            NextSequence,
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
                            "projectKey" | "project_key" => Ok(GeneratedField::ProjectKey),
                            "nextSequence" | "next_sequence" => Ok(GeneratedField::NextSequence),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchSequence;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.BatchSequence")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BatchSequence, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_key__ = None;
                let mut next_sequence__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectKey => {
                            if project_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectKey"));
                            }
                            project_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NextSequence => {
                            if next_sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextSequence"));
                            }
                            next_sequence__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BatchSequence {
                    project_key: project_key__.unwrap_or_default(),
                    next_sequence: next_sequence__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.BatchSequence", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchSupply {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.batch_key != 0 {
            len += 1;
        }
        if !self.tradable_amount.is_empty() {
            len += 1;
        }
        if !self.retired_amount.is_empty() {
            len += 1;
        }
        if !self.cancelled_amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.BatchSupply", len)?;
        if self.batch_key != 0 {
            struct_ser.serialize_field("batchKey", ToString::to_string(&self.batch_key).as_str())?;
        }
        if !self.tradable_amount.is_empty() {
            struct_ser.serialize_field("tradableAmount", &self.tradable_amount)?;
        }
        if !self.retired_amount.is_empty() {
            struct_ser.serialize_field("retiredAmount", &self.retired_amount)?;
        }
        if !self.cancelled_amount.is_empty() {
            struct_ser.serialize_field("cancelledAmount", &self.cancelled_amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchSupply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_key",
            "batchKey",
            "tradable_amount",
            "tradableAmount",
            "retired_amount",
            "retiredAmount",
            "cancelled_amount",
            "cancelledAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchKey,
            TradableAmount,
            RetiredAmount,
            CancelledAmount,
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
                            "batchKey" | "batch_key" => Ok(GeneratedField::BatchKey),
                            "tradableAmount" | "tradable_amount" => Ok(GeneratedField::TradableAmount),
                            "retiredAmount" | "retired_amount" => Ok(GeneratedField::RetiredAmount),
                            "cancelledAmount" | "cancelled_amount" => Ok(GeneratedField::CancelledAmount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchSupply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.BatchSupply")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BatchSupply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_key__ = None;
                let mut tradable_amount__ = None;
                let mut retired_amount__ = None;
                let mut cancelled_amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BatchKey => {
                            if batch_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchKey"));
                            }
                            batch_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TradableAmount => {
                            if tradable_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradableAmount"));
                            }
                            tradable_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetiredAmount => {
                            if retired_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retiredAmount"));
                            }
                            retired_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::CancelledAmount => {
                            if cancelled_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelledAmount"));
                            }
                            cancelled_amount__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BatchSupply {
                    batch_key: batch_key__.unwrap_or_default(),
                    tradable_amount: tradable_amount__.unwrap_or_default(),
                    retired_amount: retired_amount__.unwrap_or_default(),
                    cancelled_amount: cancelled_amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.BatchSupply", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Class {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key != 0 {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if !self.credit_type_abbrev.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.Class", len)?;
        if self.key != 0 {
            struct_ser.serialize_field("key", ToString::to_string(&self.key).as_str())?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", pbjson::private::base64::encode(&self.admin).as_str())?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if !self.credit_type_abbrev.is_empty() {
            struct_ser.serialize_field("creditTypeAbbrev", &self.credit_type_abbrev)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Class {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "id",
            "admin",
            "metadata",
            "credit_type_abbrev",
            "creditTypeAbbrev",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Id,
            Admin,
            Metadata,
            CreditTypeAbbrev,
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
                            "key" => Ok(GeneratedField::Key),
                            "id" => Ok(GeneratedField::Id),
                            "admin" => Ok(GeneratedField::Admin),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "creditTypeAbbrev" | "credit_type_abbrev" => Ok(GeneratedField::CreditTypeAbbrev),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Class;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.Class")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Class, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut id__ = None;
                let mut admin__ = None;
                let mut metadata__ = None;
                let mut credit_type_abbrev__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreditTypeAbbrev => {
                            if credit_type_abbrev__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditTypeAbbrev"));
                            }
                            credit_type_abbrev__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Class {
                    key: key__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    credit_type_abbrev: credit_type_abbrev__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.Class", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClassCreatorAllowlist {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enabled {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.ClassCreatorAllowlist", len)?;
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClassCreatorAllowlist {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enabled,
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
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClassCreatorAllowlist;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.ClassCreatorAllowlist")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClassCreatorAllowlist, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enabled__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClassCreatorAllowlist {
                    enabled: enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.ClassCreatorAllowlist", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClassFee {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.ClassFee", len)?;
        if let Some(v) = self.fee.as_ref() {
            struct_ser.serialize_field("fee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClassFee {
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
            type Value = ClassFee;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.ClassFee")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClassFee, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = map.next_value()?;
                        }
                    }
                }
                Ok(ClassFee {
                    fee: fee__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.ClassFee", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClassInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if !self.credit_type_abbrev.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.ClassInfo", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if !self.credit_type_abbrev.is_empty() {
            struct_ser.serialize_field("creditTypeAbbrev", &self.credit_type_abbrev)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClassInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "admin",
            "metadata",
            "credit_type_abbrev",
            "creditTypeAbbrev",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Admin,
            Metadata,
            CreditTypeAbbrev,
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
                            "admin" => Ok(GeneratedField::Admin),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "creditTypeAbbrev" | "credit_type_abbrev" => Ok(GeneratedField::CreditTypeAbbrev),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClassInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.ClassInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClassInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut admin__ = None;
                let mut metadata__ = None;
                let mut credit_type_abbrev__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreditTypeAbbrev => {
                            if credit_type_abbrev__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditTypeAbbrev"));
                            }
                            credit_type_abbrev__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ClassInfo {
                    id: id__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    credit_type_abbrev: credit_type_abbrev__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.ClassInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClassIssuer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.class_key != 0 {
            len += 1;
        }
        if !self.issuer.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.ClassIssuer", len)?;
        if self.class_key != 0 {
            struct_ser.serialize_field("classKey", ToString::to_string(&self.class_key).as_str())?;
        }
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", pbjson::private::base64::encode(&self.issuer).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClassIssuer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_key",
            "classKey",
            "issuer",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClassKey,
            Issuer,
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
                            "classKey" | "class_key" => Ok(GeneratedField::ClassKey),
                            "issuer" => Ok(GeneratedField::Issuer),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClassIssuer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.ClassIssuer")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClassIssuer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_key__ = None;
                let mut issuer__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassKey => {
                            if class_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classKey"));
                            }
                            class_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ClassIssuer {
                    class_key: class_key__.unwrap_or_default(),
                    issuer: issuer__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.ClassIssuer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClassSequence {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.credit_type_abbrev.is_empty() {
            len += 1;
        }
        if self.next_sequence != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.ClassSequence", len)?;
        if !self.credit_type_abbrev.is_empty() {
            struct_ser.serialize_field("creditTypeAbbrev", &self.credit_type_abbrev)?;
        }
        if self.next_sequence != 0 {
            struct_ser.serialize_field("nextSequence", ToString::to_string(&self.next_sequence).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClassSequence {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "credit_type_abbrev",
            "creditTypeAbbrev",
            "next_sequence",
            "nextSequence",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreditTypeAbbrev,
            NextSequence,
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
                            "creditTypeAbbrev" | "credit_type_abbrev" => Ok(GeneratedField::CreditTypeAbbrev),
                            "nextSequence" | "next_sequence" => Ok(GeneratedField::NextSequence),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClassSequence;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.ClassSequence")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ClassSequence, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credit_type_abbrev__ = None;
                let mut next_sequence__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CreditTypeAbbrev => {
                            if credit_type_abbrev__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditTypeAbbrev"));
                            }
                            credit_type_abbrev__ = Some(map.next_value()?);
                        }
                        GeneratedField::NextSequence => {
                            if next_sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextSequence"));
                            }
                            next_sequence__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ClassSequence {
                    credit_type_abbrev: credit_type_abbrev__.unwrap_or_default(),
                    next_sequence: next_sequence__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.ClassSequence", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreditType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.abbreviation.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        if self.precision != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.CreditType", len)?;
        if !self.abbreviation.is_empty() {
            struct_ser.serialize_field("abbreviation", &self.abbreviation)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.unit.is_empty() {
            struct_ser.serialize_field("unit", &self.unit)?;
        }
        if self.precision != 0 {
            struct_ser.serialize_field("precision", &self.precision)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreditType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "abbreviation",
            "name",
            "unit",
            "precision",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Abbreviation,
            Name,
            Unit,
            Precision,
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
                            "abbreviation" => Ok(GeneratedField::Abbreviation),
                            "name" => Ok(GeneratedField::Name),
                            "unit" => Ok(GeneratedField::Unit),
                            "precision" => Ok(GeneratedField::Precision),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreditType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.CreditType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreditType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut abbreviation__ = None;
                let mut name__ = None;
                let mut unit__ = None;
                let mut precision__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Abbreviation => {
                            if abbreviation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abbreviation"));
                            }
                            abbreviation__ = Some(map.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map.next_value()?);
                        }
                        GeneratedField::Precision => {
                            if precision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("precision"));
                            }
                            precision__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CreditType {
                    abbreviation: abbreviation__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                    precision: precision__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.CreditType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreditTypeProposal {
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
        if self.credit_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.CreditTypeProposal", len)?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.credit_type.as_ref() {
            struct_ser.serialize_field("creditType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreditTypeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "credit_type",
            "creditType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            CreditType,
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
                            "creditType" | "credit_type" => Ok(GeneratedField::CreditType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreditTypeProposal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.CreditTypeProposal")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CreditTypeProposal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut credit_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreditType => {
                            if credit_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditType"));
                            }
                            credit_type__ = map.next_value()?;
                        }
                    }
                }
                Ok(CreditTypeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    credit_type: credit_type__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.CreditTypeProposal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Credits {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.Credits", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Credits {
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
            type Value = Credits;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.Credits")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Credits, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                let mut amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Credits {
                    batch_denom: batch_denom__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.Credits", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventAddCreditType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.abbreviation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventAddCreditType", len)?;
        if !self.abbreviation.is_empty() {
            struct_ser.serialize_field("abbreviation", &self.abbreviation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventAddCreditType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "abbreviation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Abbreviation,
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
                            "abbreviation" => Ok(GeneratedField::Abbreviation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventAddCreditType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventAddCreditType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventAddCreditType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut abbreviation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Abbreviation => {
                            if abbreviation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abbreviation"));
                            }
                            abbreviation__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventAddCreditType {
                    abbreviation: abbreviation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventAddCreditType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventBridge {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.target.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventBridge", len)?;
        if !self.target.is_empty() {
            struct_ser.serialize_field("target", &self.target)?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventBridge {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "target",
            "recipient",
            "contract",
            "amount",
            "owner",
            "batch_denom",
            "batchDenom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Target,
            Recipient,
            Contract,
            Amount,
            Owner,
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
                            "target" => Ok(GeneratedField::Target),
                            "recipient" => Ok(GeneratedField::Recipient),
                            "contract" => Ok(GeneratedField::Contract),
                            "amount" => Ok(GeneratedField::Amount),
                            "owner" => Ok(GeneratedField::Owner),
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
            type Value = EventBridge;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventBridge")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventBridge, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut target__ = None;
                let mut recipient__ = None;
                let mut contract__ = None;
                let mut amount__ = None;
                let mut owner__ = None;
                let mut batch_denom__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = Some(map.next_value()?);
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventBridge {
                    target: target__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventBridge", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventBridgeReceive {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if self.origin_tx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventBridgeReceive", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if let Some(v) = self.origin_tx.as_ref() {
            struct_ser.serialize_field("originTx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventBridgeReceive {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "batch_denom",
            "batchDenom",
            "amount",
            "origin_tx",
            "originTx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
            BatchDenom,
            Amount,
            OriginTx,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "amount" => Ok(GeneratedField::Amount),
                            "originTx" | "origin_tx" => Ok(GeneratedField::OriginTx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventBridgeReceive;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventBridgeReceive")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventBridgeReceive, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut batch_denom__ = None;
                let mut amount__ = None;
                let mut origin_tx__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::OriginTx => {
                            if origin_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originTx"));
                            }
                            origin_tx__ = map.next_value()?;
                        }
                    }
                }
                Ok(EventBridgeReceive {
                    project_id: project_id__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    origin_tx: origin_tx__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventBridgeReceive", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventBurnRegen {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.burner.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventBurnRegen", len)?;
        if !self.burner.is_empty() {
            struct_ser.serialize_field("burner", &self.burner)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventBurnRegen {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "burner",
            "amount",
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Burner,
            Amount,
            Reason,
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
                            "burner" => Ok(GeneratedField::Burner),
                            "amount" => Ok(GeneratedField::Amount),
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventBurnRegen;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventBurnRegen")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventBurnRegen, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut burner__ = None;
                let mut amount__ = None;
                let mut reason__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Burner => {
                            if burner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("burner"));
                            }
                            burner__ = Some(map.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventBurnRegen {
                    burner: burner__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventBurnRegen", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventCancel {
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
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventCancel", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventCancel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "owner",
            "batch_denom",
            "batchDenom",
            "amount",
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            BatchDenom,
            Amount,
            Reason,
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
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "amount" => Ok(GeneratedField::Amount),
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventCancel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventCancel")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventCancel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut batch_denom__ = None;
                let mut amount__ = None;
                let mut reason__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventCancel {
                    owner: owner__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventCancel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventCreateBatch {
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
        if self.origin_tx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventCreateBatch", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if let Some(v) = self.origin_tx.as_ref() {
            struct_ser.serialize_field("originTx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventCreateBatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_denom",
            "batchDenom",
            "origin_tx",
            "originTx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchDenom,
            OriginTx,
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
                            "originTx" | "origin_tx" => Ok(GeneratedField::OriginTx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventCreateBatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventCreateBatch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventCreateBatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                let mut origin_tx__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::OriginTx => {
                            if origin_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originTx"));
                            }
                            origin_tx__ = map.next_value()?;
                        }
                    }
                }
                Ok(EventCreateBatch {
                    batch_denom: batch_denom__.unwrap_or_default(),
                    origin_tx: origin_tx__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventCreateBatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventCreateClass {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.class_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventCreateClass", len)?;
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventCreateClass {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_id",
            "classId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = EventCreateClass;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventCreateClass")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventCreateClass, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventCreateClass {
                    class_id: class_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventCreateClass", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventCreateProject {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventCreateProject", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventCreateProject {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventCreateProject;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventCreateProject")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventCreateProject, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventCreateProject {
                    project_id: project_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventCreateProject", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventMint {
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
        if !self.tradable_amount.is_empty() {
            len += 1;
        }
        if !self.retired_amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventMint", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.tradable_amount.is_empty() {
            struct_ser.serialize_field("tradableAmount", &self.tradable_amount)?;
        }
        if !self.retired_amount.is_empty() {
            struct_ser.serialize_field("retiredAmount", &self.retired_amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventMint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_denom",
            "batchDenom",
            "tradable_amount",
            "tradableAmount",
            "retired_amount",
            "retiredAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchDenom,
            TradableAmount,
            RetiredAmount,
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
                            "tradableAmount" | "tradable_amount" => Ok(GeneratedField::TradableAmount),
                            "retiredAmount" | "retired_amount" => Ok(GeneratedField::RetiredAmount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventMint;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventMint")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventMint, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                let mut tradable_amount__ = None;
                let mut retired_amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::TradableAmount => {
                            if tradable_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradableAmount"));
                            }
                            tradable_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetiredAmount => {
                            if retired_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retiredAmount"));
                            }
                            retired_amount__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventMint {
                    batch_denom: batch_denom__.unwrap_or_default(),
                    tradable_amount: tradable_amount__.unwrap_or_default(),
                    retired_amount: retired_amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventMint", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventMintBatchCredits {
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
        if self.origin_tx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventMintBatchCredits", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if let Some(v) = self.origin_tx.as_ref() {
            struct_ser.serialize_field("originTx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventMintBatchCredits {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_denom",
            "batchDenom",
            "origin_tx",
            "originTx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchDenom,
            OriginTx,
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
                            "originTx" | "origin_tx" => Ok(GeneratedField::OriginTx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventMintBatchCredits;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventMintBatchCredits")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventMintBatchCredits, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                let mut origin_tx__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::OriginTx => {
                            if origin_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originTx"));
                            }
                            origin_tx__ = map.next_value()?;
                        }
                    }
                }
                Ok(EventMintBatchCredits {
                    batch_denom: batch_denom__.unwrap_or_default(),
                    origin_tx: origin_tx__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventMintBatchCredits", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventRetire {
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
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.jurisdiction.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventRetire", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.jurisdiction.is_empty() {
            struct_ser.serialize_field("jurisdiction", &self.jurisdiction)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventRetire {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "owner",
            "batch_denom",
            "batchDenom",
            "amount",
            "jurisdiction",
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            BatchDenom,
            Amount,
            Jurisdiction,
            Reason,
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
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "amount" => Ok(GeneratedField::Amount),
                            "jurisdiction" => Ok(GeneratedField::Jurisdiction),
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventRetire;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventRetire")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventRetire, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut batch_denom__ = None;
                let mut amount__ = None;
                let mut jurisdiction__ = None;
                let mut reason__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::Jurisdiction => {
                            if jurisdiction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            jurisdiction__ = Some(map.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventRetire {
                    owner: owner__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    jurisdiction: jurisdiction__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventRetire", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventSealBatch {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventSealBatch", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventSealBatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_denom",
            "batchDenom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = EventSealBatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventSealBatch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventSealBatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventSealBatch {
                    batch_denom: batch_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventSealBatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventTransfer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        if !self.tradable_amount.is_empty() {
            len += 1;
        }
        if !self.retired_amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventTransfer", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.tradable_amount.is_empty() {
            struct_ser.serialize_field("tradableAmount", &self.tradable_amount)?;
        }
        if !self.retired_amount.is_empty() {
            struct_ser.serialize_field("retiredAmount", &self.retired_amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventTransfer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "recipient",
            "batch_denom",
            "batchDenom",
            "tradable_amount",
            "tradableAmount",
            "retired_amount",
            "retiredAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Recipient,
            BatchDenom,
            TradableAmount,
            RetiredAmount,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "recipient" => Ok(GeneratedField::Recipient),
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "tradableAmount" | "tradable_amount" => Ok(GeneratedField::TradableAmount),
                            "retiredAmount" | "retired_amount" => Ok(GeneratedField::RetiredAmount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventTransfer;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventTransfer")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventTransfer, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut recipient__ = None;
                let mut batch_denom__ = None;
                let mut tradable_amount__ = None;
                let mut retired_amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map.next_value()?);
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::TradableAmount => {
                            if tradable_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradableAmount"));
                            }
                            tradable_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetiredAmount => {
                            if retired_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retiredAmount"));
                            }
                            retired_amount__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventTransfer {
                    sender: sender__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                    tradable_amount: tradable_amount__.unwrap_or_default(),
                    retired_amount: retired_amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventTransfer", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventUpdateApplication {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.class_id.is_empty() {
            len += 1;
        }
        if self.action != 0 {
            len += 1;
        }
        if !self.new_application_metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventUpdateApplication", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if self.action != 0 {
            let v = event_update_application::Action::from_i32(self.action)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.action)))?;
            struct_ser.serialize_field("action", &v)?;
        }
        if !self.new_application_metadata.is_empty() {
            struct_ser.serialize_field("newApplicationMetadata", &self.new_application_metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventUpdateApplication {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "class_id",
            "classId",
            "action",
            "new_application_metadata",
            "newApplicationMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
            ClassId,
            Action,
            NewApplicationMetadata,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
                            "action" => Ok(GeneratedField::Action),
                            "newApplicationMetadata" | "new_application_metadata" => Ok(GeneratedField::NewApplicationMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventUpdateApplication;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventUpdateApplication")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventUpdateApplication, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut class_id__ = None;
                let mut action__ = None;
                let mut new_application_metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = Some(map.next_value::<event_update_application::Action>()? as i32);
                        }
                        GeneratedField::NewApplicationMetadata => {
                            if new_application_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newApplicationMetadata"));
                            }
                            new_application_metadata__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventUpdateApplication {
                    project_id: project_id__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                    action: action__.unwrap_or_default(),
                    new_application_metadata: new_application_metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventUpdateApplication", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for event_update_application::Action {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "ACTION_UNSPECIFIED",
            Self::Create => "ACTION_CREATE",
            Self::Update => "ACTION_UPDATE",
            Self::Withdraw => "ACTION_WITHDRAW",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for event_update_application::Action {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ACTION_UNSPECIFIED",
            "ACTION_CREATE",
            "ACTION_UPDATE",
            "ACTION_WITHDRAW",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = event_update_application::Action;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(event_update_application::Action::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(event_update_application::Action::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ACTION_UNSPECIFIED" => Ok(event_update_application::Action::Unspecified),
                    "ACTION_CREATE" => Ok(event_update_application::Action::Create),
                    "ACTION_UPDATE" => Ok(event_update_application::Action::Update),
                    "ACTION_WITHDRAW" => Ok(event_update_application::Action::Withdraw),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for EventUpdateBatchMetadata {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventUpdateBatchMetadata", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventUpdateBatchMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_denom",
            "batchDenom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = EventUpdateBatchMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventUpdateBatchMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventUpdateBatchMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventUpdateBatchMetadata {
                    batch_denom: batch_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventUpdateBatchMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventUpdateClassAdmin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.class_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventUpdateClassAdmin", len)?;
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventUpdateClassAdmin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_id",
            "classId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = EventUpdateClassAdmin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventUpdateClassAdmin")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventUpdateClassAdmin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventUpdateClassAdmin {
                    class_id: class_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventUpdateClassAdmin", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventUpdateClassIssuers {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.class_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventUpdateClassIssuers", len)?;
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventUpdateClassIssuers {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_id",
            "classId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = EventUpdateClassIssuers;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventUpdateClassIssuers")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventUpdateClassIssuers, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventUpdateClassIssuers {
                    class_id: class_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventUpdateClassIssuers", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventUpdateClassMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.class_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventUpdateClassMetadata", len)?;
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventUpdateClassMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_id",
            "classId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = EventUpdateClassMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventUpdateClassMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventUpdateClassMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventUpdateClassMetadata {
                    class_id: class_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventUpdateClassMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventUpdateProjectAdmin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventUpdateProjectAdmin", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventUpdateProjectAdmin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventUpdateProjectAdmin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventUpdateProjectAdmin")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventUpdateProjectAdmin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventUpdateProjectAdmin {
                    project_id: project_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventUpdateProjectAdmin", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventUpdateProjectEnrollment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.class_id.is_empty() {
            len += 1;
        }
        if self.old_status != 0 {
            len += 1;
        }
        if self.new_status != 0 {
            len += 1;
        }
        if !self.new_enrollment_metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventUpdateProjectEnrollment", len)?;
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if self.old_status != 0 {
            let v = ProjectEnrollmentStatus::from_i32(self.old_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.old_status)))?;
            struct_ser.serialize_field("oldStatus", &v)?;
        }
        if self.new_status != 0 {
            let v = ProjectEnrollmentStatus::from_i32(self.new_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.new_status)))?;
            struct_ser.serialize_field("newStatus", &v)?;
        }
        if !self.new_enrollment_metadata.is_empty() {
            struct_ser.serialize_field("newEnrollmentMetadata", &self.new_enrollment_metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventUpdateProjectEnrollment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "issuer",
            "project_id",
            "projectId",
            "class_id",
            "classId",
            "old_status",
            "oldStatus",
            "new_status",
            "newStatus",
            "new_enrollment_metadata",
            "newEnrollmentMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Issuer,
            ProjectId,
            ClassId,
            OldStatus,
            NewStatus,
            NewEnrollmentMetadata,
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
                            "issuer" => Ok(GeneratedField::Issuer),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
                            "oldStatus" | "old_status" => Ok(GeneratedField::OldStatus),
                            "newStatus" | "new_status" => Ok(GeneratedField::NewStatus),
                            "newEnrollmentMetadata" | "new_enrollment_metadata" => Ok(GeneratedField::NewEnrollmentMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventUpdateProjectEnrollment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventUpdateProjectEnrollment")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventUpdateProjectEnrollment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut issuer__ = None;
                let mut project_id__ = None;
                let mut class_id__ = None;
                let mut old_status__ = None;
                let mut new_status__ = None;
                let mut new_enrollment_metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::OldStatus => {
                            if old_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("oldStatus"));
                            }
                            old_status__ = Some(map.next_value::<ProjectEnrollmentStatus>()? as i32);
                        }
                        GeneratedField::NewStatus => {
                            if new_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newStatus"));
                            }
                            new_status__ = Some(map.next_value::<ProjectEnrollmentStatus>()? as i32);
                        }
                        GeneratedField::NewEnrollmentMetadata => {
                            if new_enrollment_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newEnrollmentMetadata"));
                            }
                            new_enrollment_metadata__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventUpdateProjectEnrollment {
                    issuer: issuer__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                    old_status: old_status__.unwrap_or_default(),
                    new_status: new_status__.unwrap_or_default(),
                    new_enrollment_metadata: new_enrollment_metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventUpdateProjectEnrollment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventUpdateProjectMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.EventUpdateProjectMetadata", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventUpdateProjectMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventUpdateProjectMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.EventUpdateProjectMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<EventUpdateProjectMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(EventUpdateProjectMetadata {
                    project_id: project_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.EventUpdateProjectMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddAllowedBridgeChain {
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
        if !self.chain_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgAddAllowedBridgeChain", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.chain_name.is_empty() {
            struct_ser.serialize_field("chainName", &self.chain_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddAllowedBridgeChain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "chain_name",
            "chainName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            ChainName,
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
                            "chainName" | "chain_name" => Ok(GeneratedField::ChainName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddAllowedBridgeChain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgAddAllowedBridgeChain")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgAddAllowedBridgeChain, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut chain_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChainName => {
                            if chain_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainName"));
                            }
                            chain_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgAddAllowedBridgeChain {
                    authority: authority__.unwrap_or_default(),
                    chain_name: chain_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgAddAllowedBridgeChain", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddAllowedBridgeChainResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgAddAllowedBridgeChainResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddAllowedBridgeChainResponse {
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
            type Value = MsgAddAllowedBridgeChainResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgAddAllowedBridgeChainResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgAddAllowedBridgeChainResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAddAllowedBridgeChainResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgAddAllowedBridgeChainResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddClassCreator {
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
        if !self.creator.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgAddClassCreator", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.creator.is_empty() {
            struct_ser.serialize_field("creator", &self.creator)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddClassCreator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "creator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Creator,
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
                            "creator" => Ok(GeneratedField::Creator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddClassCreator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgAddClassCreator")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgAddClassCreator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut creator__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                        GeneratedField::Creator => {
                            if creator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creator"));
                            }
                            creator__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgAddClassCreator {
                    authority: authority__.unwrap_or_default(),
                    creator: creator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgAddClassCreator", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddClassCreatorResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgAddClassCreatorResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddClassCreatorResponse {
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
            type Value = MsgAddClassCreatorResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgAddClassCreatorResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgAddClassCreatorResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAddClassCreatorResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgAddClassCreatorResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddCreditType {
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
        if self.credit_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgAddCreditType", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.credit_type.as_ref() {
            struct_ser.serialize_field("creditType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddCreditType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "credit_type",
            "creditType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            CreditType,
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
                            "creditType" | "credit_type" => Ok(GeneratedField::CreditType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddCreditType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgAddCreditType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgAddCreditType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut credit_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreditType => {
                            if credit_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditType"));
                            }
                            credit_type__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgAddCreditType {
                    authority: authority__.unwrap_or_default(),
                    credit_type: credit_type__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgAddCreditType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgAddCreditTypeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgAddCreditTypeResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddCreditTypeResponse {
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
            type Value = MsgAddCreditTypeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgAddCreditTypeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgAddCreditTypeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAddCreditTypeResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgAddCreditTypeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgBridge {
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
        if !self.target.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.credits.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgBridge", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.target.is_empty() {
            struct_ser.serialize_field("target", &self.target)?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.credits.is_empty() {
            struct_ser.serialize_field("credits", &self.credits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgBridge {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "owner",
            "target",
            "recipient",
            "credits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            Target,
            Recipient,
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
                            "target" => Ok(GeneratedField::Target),
                            "recipient" => Ok(GeneratedField::Recipient),
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
            type Value = MsgBridge;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgBridge")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgBridge, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut target__ = None;
                let mut recipient__ = None;
                let mut credits__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map.next_value()?);
                        }
                        GeneratedField::Target => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("target"));
                            }
                            target__ = Some(map.next_value()?);
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map.next_value()?);
                        }
                        GeneratedField::Credits => {
                            if credits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credits"));
                            }
                            credits__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgBridge {
                    owner: owner__.unwrap_or_default(),
                    target: target__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    credits: credits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgBridge", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgBridgeReceive {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.class_id.is_empty() {
            len += 1;
        }
        if self.project.is_some() {
            len += 1;
        }
        if self.batch.is_some() {
            len += 1;
        }
        if self.origin_tx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgBridgeReceive", len)?;
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if let Some(v) = self.project.as_ref() {
            struct_ser.serialize_field("project", v)?;
        }
        if let Some(v) = self.batch.as_ref() {
            struct_ser.serialize_field("batch", v)?;
        }
        if let Some(v) = self.origin_tx.as_ref() {
            struct_ser.serialize_field("originTx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgBridgeReceive {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "issuer",
            "class_id",
            "classId",
            "project",
            "batch",
            "origin_tx",
            "originTx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Issuer,
            ClassId,
            Project,
            Batch,
            OriginTx,
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
                            "issuer" => Ok(GeneratedField::Issuer),
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
                            "project" => Ok(GeneratedField::Project),
                            "batch" => Ok(GeneratedField::Batch),
                            "originTx" | "origin_tx" => Ok(GeneratedField::OriginTx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgBridgeReceive;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgBridgeReceive")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgBridgeReceive, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut issuer__ = None;
                let mut class_id__ = None;
                let mut project__ = None;
                let mut batch__ = None;
                let mut origin_tx__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Project => {
                            if project__.is_some() {
                                return Err(serde::de::Error::duplicate_field("project"));
                            }
                            project__ = map.next_value()?;
                        }
                        GeneratedField::Batch => {
                            if batch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batch"));
                            }
                            batch__ = map.next_value()?;
                        }
                        GeneratedField::OriginTx => {
                            if origin_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originTx"));
                            }
                            origin_tx__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgBridgeReceive {
                    issuer: issuer__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                    project: project__,
                    batch: batch__,
                    origin_tx: origin_tx__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgBridgeReceive", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for msg_bridge_receive::Batch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if self.start_date.is_some() {
            len += 1;
        }
        if self.end_date.is_some() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgBridgeReceive.Batch", len)?;
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if let Some(v) = self.start_date.as_ref() {
            struct_ser.serialize_field("startDate", v)?;
        }
        if let Some(v) = self.end_date.as_ref() {
            struct_ser.serialize_field("endDate", v)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for msg_bridge_receive::Batch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "recipient",
            "amount",
            "start_date",
            "startDate",
            "end_date",
            "endDate",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Recipient,
            Amount,
            StartDate,
            EndDate,
            Metadata,
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
                            "recipient" => Ok(GeneratedField::Recipient),
                            "amount" => Ok(GeneratedField::Amount),
                            "startDate" | "start_date" => Ok(GeneratedField::StartDate),
                            "endDate" | "end_date" => Ok(GeneratedField::EndDate),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = msg_bridge_receive::Batch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgBridgeReceive.Batch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<msg_bridge_receive::Batch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut recipient__ = None;
                let mut amount__ = None;
                let mut start_date__ = None;
                let mut end_date__ = None;
                let mut metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::StartDate => {
                            if start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startDate"));
                            }
                            start_date__ = map.next_value()?;
                        }
                        GeneratedField::EndDate => {
                            if end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endDate"));
                            }
                            end_date__ = map.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(msg_bridge_receive::Batch {
                    recipient: recipient__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    start_date: start_date__,
                    end_date: end_date__,
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgBridgeReceive.Batch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for msg_bridge_receive::Project {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reference_id.is_empty() {
            len += 1;
        }
        if !self.jurisdiction.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgBridgeReceive.Project", len)?;
        if !self.reference_id.is_empty() {
            struct_ser.serialize_field("referenceId", &self.reference_id)?;
        }
        if !self.jurisdiction.is_empty() {
            struct_ser.serialize_field("jurisdiction", &self.jurisdiction)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for msg_bridge_receive::Project {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reference_id",
            "referenceId",
            "jurisdiction",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReferenceId,
            Jurisdiction,
            Metadata,
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
                            "referenceId" | "reference_id" => Ok(GeneratedField::ReferenceId),
                            "jurisdiction" => Ok(GeneratedField::Jurisdiction),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = msg_bridge_receive::Project;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgBridgeReceive.Project")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<msg_bridge_receive::Project, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reference_id__ = None;
                let mut jurisdiction__ = None;
                let mut metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ReferenceId => {
                            if reference_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceId"));
                            }
                            reference_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Jurisdiction => {
                            if jurisdiction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            jurisdiction__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(msg_bridge_receive::Project {
                    reference_id: reference_id__.unwrap_or_default(),
                    jurisdiction: jurisdiction__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgBridgeReceive.Project", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgBridgeReceiveResponse {
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
        if !self.project_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgBridgeReceiveResponse", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgBridgeReceiveResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_denom",
            "batchDenom",
            "project_id",
            "projectId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchDenom,
            ProjectId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgBridgeReceiveResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgBridgeReceiveResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgBridgeReceiveResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                let mut project_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgBridgeReceiveResponse {
                    batch_denom: batch_denom__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgBridgeReceiveResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgBridgeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgBridgeResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgBridgeResponse {
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
            type Value = MsgBridgeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgBridgeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgBridgeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgBridgeResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgBridgeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgBurnRegen {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.burner.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgBurnRegen", len)?;
        if !self.burner.is_empty() {
            struct_ser.serialize_field("burner", &self.burner)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgBurnRegen {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "burner",
            "amount",
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Burner,
            Amount,
            Reason,
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
                            "burner" => Ok(GeneratedField::Burner),
                            "amount" => Ok(GeneratedField::Amount),
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgBurnRegen;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgBurnRegen")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgBurnRegen, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut burner__ = None;
                let mut amount__ = None;
                let mut reason__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Burner => {
                            if burner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("burner"));
                            }
                            burner__ = Some(map.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgBurnRegen {
                    burner: burner__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgBurnRegen", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgBurnRegenResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgBurnRegenResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgBurnRegenResponse {
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
            type Value = MsgBurnRegenResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgBurnRegenResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgBurnRegenResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgBurnRegenResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgBurnRegenResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCancel {
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
        if !self.credits.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgCancel", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.credits.is_empty() {
            struct_ser.serialize_field("credits", &self.credits)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCancel {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "owner",
            "credits",
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            Credits,
            Reason,
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
                            "credits" => Ok(GeneratedField::Credits),
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCancel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgCancel")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgCancel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut credits__ = None;
                let mut reason__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map.next_value()?);
                        }
                        GeneratedField::Credits => {
                            if credits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credits"));
                            }
                            credits__ = Some(map.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgCancel {
                    owner: owner__.unwrap_or_default(),
                    credits: credits__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgCancel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCancelResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgCancelResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCancelResponse {
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
            type Value = MsgCancelResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgCancelResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgCancelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCancelResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgCancelResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCreateBatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.issuance.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.start_date.is_some() {
            len += 1;
        }
        if self.end_date.is_some() {
            len += 1;
        }
        if self.open {
            len += 1;
        }
        if self.origin_tx.is_some() {
            len += 1;
        }
        if !self.class_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgCreateBatch", len)?;
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.issuance.is_empty() {
            struct_ser.serialize_field("issuance", &self.issuance)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if let Some(v) = self.start_date.as_ref() {
            struct_ser.serialize_field("startDate", v)?;
        }
        if let Some(v) = self.end_date.as_ref() {
            struct_ser.serialize_field("endDate", v)?;
        }
        if self.open {
            struct_ser.serialize_field("open", &self.open)?;
        }
        if let Some(v) = self.origin_tx.as_ref() {
            struct_ser.serialize_field("originTx", v)?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateBatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "issuer",
            "project_id",
            "projectId",
            "issuance",
            "metadata",
            "start_date",
            "startDate",
            "end_date",
            "endDate",
            "open",
            "origin_tx",
            "originTx",
            "class_id",
            "classId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Issuer,
            ProjectId,
            Issuance,
            Metadata,
            StartDate,
            EndDate,
            Open,
            OriginTx,
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
                            "issuer" => Ok(GeneratedField::Issuer),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "issuance" => Ok(GeneratedField::Issuance),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "startDate" | "start_date" => Ok(GeneratedField::StartDate),
                            "endDate" | "end_date" => Ok(GeneratedField::EndDate),
                            "open" => Ok(GeneratedField::Open),
                            "originTx" | "origin_tx" => Ok(GeneratedField::OriginTx),
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
            type Value = MsgCreateBatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgCreateBatch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgCreateBatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut issuer__ = None;
                let mut project_id__ = None;
                let mut issuance__ = None;
                let mut metadata__ = None;
                let mut start_date__ = None;
                let mut end_date__ = None;
                let mut open__ = None;
                let mut origin_tx__ = None;
                let mut class_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Issuance => {
                            if issuance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuance"));
                            }
                            issuance__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::StartDate => {
                            if start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startDate"));
                            }
                            start_date__ = map.next_value()?;
                        }
                        GeneratedField::EndDate => {
                            if end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endDate"));
                            }
                            end_date__ = map.next_value()?;
                        }
                        GeneratedField::Open => {
                            if open__.is_some() {
                                return Err(serde::de::Error::duplicate_field("open"));
                            }
                            open__ = Some(map.next_value()?);
                        }
                        GeneratedField::OriginTx => {
                            if origin_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originTx"));
                            }
                            origin_tx__ = map.next_value()?;
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgCreateBatch {
                    issuer: issuer__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    issuance: issuance__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    start_date: start_date__,
                    end_date: end_date__,
                    open: open__.unwrap_or_default(),
                    origin_tx: origin_tx__,
                    class_id: class_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgCreateBatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCreateBatchResponse {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgCreateBatchResponse", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateBatchResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_denom",
            "batchDenom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = MsgCreateBatchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgCreateBatchResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgCreateBatchResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgCreateBatchResponse {
                    batch_denom: batch_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgCreateBatchResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCreateClass {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.issuers.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if !self.credit_type_abbrev.is_empty() {
            len += 1;
        }
        if self.fee.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgCreateClass", len)?;
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.issuers.is_empty() {
            struct_ser.serialize_field("issuers", &self.issuers)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if !self.credit_type_abbrev.is_empty() {
            struct_ser.serialize_field("creditTypeAbbrev", &self.credit_type_abbrev)?;
        }
        if let Some(v) = self.fee.as_ref() {
            struct_ser.serialize_field("fee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateClass {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "admin",
            "issuers",
            "metadata",
            "credit_type_abbrev",
            "creditTypeAbbrev",
            "fee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Admin,
            Issuers,
            Metadata,
            CreditTypeAbbrev,
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
                            "admin" => Ok(GeneratedField::Admin),
                            "issuers" => Ok(GeneratedField::Issuers),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "creditTypeAbbrev" | "credit_type_abbrev" => Ok(GeneratedField::CreditTypeAbbrev),
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
            type Value = MsgCreateClass;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgCreateClass")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgCreateClass, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut admin__ = None;
                let mut issuers__ = None;
                let mut metadata__ = None;
                let mut credit_type_abbrev__ = None;
                let mut fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map.next_value()?);
                        }
                        GeneratedField::Issuers => {
                            if issuers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuers"));
                            }
                            issuers__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreditTypeAbbrev => {
                            if credit_type_abbrev__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditTypeAbbrev"));
                            }
                            credit_type_abbrev__ = Some(map.next_value()?);
                        }
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgCreateClass {
                    admin: admin__.unwrap_or_default(),
                    issuers: issuers__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    credit_type_abbrev: credit_type_abbrev__.unwrap_or_default(),
                    fee: fee__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgCreateClass", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCreateClassResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.class_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgCreateClassResponse", len)?;
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateClassResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_id",
            "classId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = MsgCreateClassResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgCreateClassResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgCreateClassResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgCreateClassResponse {
                    class_id: class_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgCreateClassResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCreateOrUpdateApplication {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_admin.is_empty() {
            len += 1;
        }
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.class_id.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.withdraw {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgCreateOrUpdateApplication", len)?;
        if !self.project_admin.is_empty() {
            struct_ser.serialize_field("projectAdmin", &self.project_admin)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if self.withdraw {
            struct_ser.serialize_field("withdraw", &self.withdraw)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateOrUpdateApplication {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_admin",
            "projectAdmin",
            "project_id",
            "projectId",
            "class_id",
            "classId",
            "metadata",
            "withdraw",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectAdmin,
            ProjectId,
            ClassId,
            Metadata,
            Withdraw,
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
                            "projectAdmin" | "project_admin" => Ok(GeneratedField::ProjectAdmin),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "withdraw" => Ok(GeneratedField::Withdraw),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreateOrUpdateApplication;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgCreateOrUpdateApplication")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgCreateOrUpdateApplication, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_admin__ = None;
                let mut project_id__ = None;
                let mut class_id__ = None;
                let mut metadata__ = None;
                let mut withdraw__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectAdmin => {
                            if project_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectAdmin"));
                            }
                            project_admin__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::Withdraw => {
                            if withdraw__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withdraw"));
                            }
                            withdraw__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgCreateOrUpdateApplication {
                    project_admin: project_admin__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    withdraw: withdraw__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgCreateOrUpdateApplication", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCreateOrUpdateApplicationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgCreateOrUpdateApplicationResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateOrUpdateApplicationResponse {
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
            type Value = MsgCreateOrUpdateApplicationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgCreateOrUpdateApplicationResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgCreateOrUpdateApplicationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCreateOrUpdateApplicationResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgCreateOrUpdateApplicationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCreateProject {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.class_id.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if !self.jurisdiction.is_empty() {
            len += 1;
        }
        if !self.reference_id.is_empty() {
            len += 1;
        }
        if self.fee.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgCreateProject", len)?;
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if !self.jurisdiction.is_empty() {
            struct_ser.serialize_field("jurisdiction", &self.jurisdiction)?;
        }
        if !self.reference_id.is_empty() {
            struct_ser.serialize_field("referenceId", &self.reference_id)?;
        }
        if let Some(v) = self.fee.as_ref() {
            struct_ser.serialize_field("fee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateProject {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "admin",
            "class_id",
            "classId",
            "metadata",
            "jurisdiction",
            "reference_id",
            "referenceId",
            "fee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Admin,
            ClassId,
            Metadata,
            Jurisdiction,
            ReferenceId,
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
                            "admin" => Ok(GeneratedField::Admin),
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "jurisdiction" => Ok(GeneratedField::Jurisdiction),
                            "referenceId" | "reference_id" => Ok(GeneratedField::ReferenceId),
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
            type Value = MsgCreateProject;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgCreateProject")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgCreateProject, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut admin__ = None;
                let mut class_id__ = None;
                let mut metadata__ = None;
                let mut jurisdiction__ = None;
                let mut reference_id__ = None;
                let mut fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::Jurisdiction => {
                            if jurisdiction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            jurisdiction__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReferenceId => {
                            if reference_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceId"));
                            }
                            reference_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgCreateProject {
                    admin: admin__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    jurisdiction: jurisdiction__.unwrap_or_default(),
                    reference_id: reference_id__.unwrap_or_default(),
                    fee: fee__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgCreateProject", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCreateProjectResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgCreateProjectResponse", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateProjectResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreateProjectResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgCreateProjectResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgCreateProjectResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgCreateProjectResponse {
                    project_id: project_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgCreateProjectResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCreateUnregisteredProject {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if !self.jurisdiction.is_empty() {
            len += 1;
        }
        if !self.reference_id.is_empty() {
            len += 1;
        }
        if self.fee.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgCreateUnregisteredProject", len)?;
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if !self.jurisdiction.is_empty() {
            struct_ser.serialize_field("jurisdiction", &self.jurisdiction)?;
        }
        if !self.reference_id.is_empty() {
            struct_ser.serialize_field("referenceId", &self.reference_id)?;
        }
        if let Some(v) = self.fee.as_ref() {
            struct_ser.serialize_field("fee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateUnregisteredProject {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "admin",
            "metadata",
            "jurisdiction",
            "reference_id",
            "referenceId",
            "fee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Admin,
            Metadata,
            Jurisdiction,
            ReferenceId,
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
                            "admin" => Ok(GeneratedField::Admin),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "jurisdiction" => Ok(GeneratedField::Jurisdiction),
                            "referenceId" | "reference_id" => Ok(GeneratedField::ReferenceId),
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
            type Value = MsgCreateUnregisteredProject;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgCreateUnregisteredProject")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgCreateUnregisteredProject, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut admin__ = None;
                let mut metadata__ = None;
                let mut jurisdiction__ = None;
                let mut reference_id__ = None;
                let mut fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::Jurisdiction => {
                            if jurisdiction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            jurisdiction__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReferenceId => {
                            if reference_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceId"));
                            }
                            reference_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgCreateUnregisteredProject {
                    admin: admin__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    jurisdiction: jurisdiction__.unwrap_or_default(),
                    reference_id: reference_id__.unwrap_or_default(),
                    fee: fee__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgCreateUnregisteredProject", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgCreateUnregisteredProjectResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgCreateUnregisteredProjectResponse", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateUnregisteredProjectResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgCreateUnregisteredProjectResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgCreateUnregisteredProjectResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgCreateUnregisteredProjectResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgCreateUnregisteredProjectResponse {
                    project_id: project_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgCreateUnregisteredProjectResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgMintBatchCredits {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        if !self.issuance.is_empty() {
            len += 1;
        }
        if self.origin_tx.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgMintBatchCredits", len)?;
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.issuance.is_empty() {
            struct_ser.serialize_field("issuance", &self.issuance)?;
        }
        if let Some(v) = self.origin_tx.as_ref() {
            struct_ser.serialize_field("originTx", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgMintBatchCredits {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "issuer",
            "batch_denom",
            "batchDenom",
            "issuance",
            "origin_tx",
            "originTx",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Issuer,
            BatchDenom,
            Issuance,
            OriginTx,
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
                            "issuer" => Ok(GeneratedField::Issuer),
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "issuance" => Ok(GeneratedField::Issuance),
                            "originTx" | "origin_tx" => Ok(GeneratedField::OriginTx),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgMintBatchCredits;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgMintBatchCredits")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgMintBatchCredits, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut issuer__ = None;
                let mut batch_denom__ = None;
                let mut issuance__ = None;
                let mut origin_tx__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::Issuance => {
                            if issuance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuance"));
                            }
                            issuance__ = Some(map.next_value()?);
                        }
                        GeneratedField::OriginTx => {
                            if origin_tx__.is_some() {
                                return Err(serde::de::Error::duplicate_field("originTx"));
                            }
                            origin_tx__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgMintBatchCredits {
                    issuer: issuer__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                    issuance: issuance__.unwrap_or_default(),
                    origin_tx: origin_tx__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgMintBatchCredits", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgMintBatchCreditsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgMintBatchCreditsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgMintBatchCreditsResponse {
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
            type Value = MsgMintBatchCreditsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgMintBatchCreditsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgMintBatchCreditsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgMintBatchCreditsResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgMintBatchCreditsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRemoveAllowedBridgeChain {
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
        if !self.chain_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgRemoveAllowedBridgeChain", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.chain_name.is_empty() {
            struct_ser.serialize_field("chainName", &self.chain_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveAllowedBridgeChain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "chain_name",
            "chainName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            ChainName,
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
                            "chainName" | "chain_name" => Ok(GeneratedField::ChainName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemoveAllowedBridgeChain;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgRemoveAllowedBridgeChain")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgRemoveAllowedBridgeChain, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut chain_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChainName => {
                            if chain_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainName"));
                            }
                            chain_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgRemoveAllowedBridgeChain {
                    authority: authority__.unwrap_or_default(),
                    chain_name: chain_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgRemoveAllowedBridgeChain", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRemoveAllowedBridgeChainResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgRemoveAllowedBridgeChainResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveAllowedBridgeChainResponse {
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
            type Value = MsgRemoveAllowedBridgeChainResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgRemoveAllowedBridgeChainResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgRemoveAllowedBridgeChainResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemoveAllowedBridgeChainResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgRemoveAllowedBridgeChainResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRemoveClassCreator {
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
        if !self.creator.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgRemoveClassCreator", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.creator.is_empty() {
            struct_ser.serialize_field("creator", &self.creator)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveClassCreator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "creator",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Creator,
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
                            "creator" => Ok(GeneratedField::Creator),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRemoveClassCreator;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgRemoveClassCreator")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgRemoveClassCreator, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut creator__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                        GeneratedField::Creator => {
                            if creator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creator"));
                            }
                            creator__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgRemoveClassCreator {
                    authority: authority__.unwrap_or_default(),
                    creator: creator__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgRemoveClassCreator", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRemoveClassCreatorResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgRemoveClassCreatorResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRemoveClassCreatorResponse {
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
            type Value = MsgRemoveClassCreatorResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgRemoveClassCreatorResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgRemoveClassCreatorResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRemoveClassCreatorResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgRemoveClassCreatorResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRetire {
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
        if !self.credits.is_empty() {
            len += 1;
        }
        if !self.jurisdiction.is_empty() {
            len += 1;
        }
        if !self.reason.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgRetire", len)?;
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.credits.is_empty() {
            struct_ser.serialize_field("credits", &self.credits)?;
        }
        if !self.jurisdiction.is_empty() {
            struct_ser.serialize_field("jurisdiction", &self.jurisdiction)?;
        }
        if !self.reason.is_empty() {
            struct_ser.serialize_field("reason", &self.reason)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRetire {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "owner",
            "credits",
            "jurisdiction",
            "reason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Owner,
            Credits,
            Jurisdiction,
            Reason,
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
                            "credits" => Ok(GeneratedField::Credits),
                            "jurisdiction" => Ok(GeneratedField::Jurisdiction),
                            "reason" => Ok(GeneratedField::Reason),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRetire;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgRetire")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgRetire, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut owner__ = None;
                let mut credits__ = None;
                let mut jurisdiction__ = None;
                let mut reason__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map.next_value()?);
                        }
                        GeneratedField::Credits => {
                            if credits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credits"));
                            }
                            credits__ = Some(map.next_value()?);
                        }
                        GeneratedField::Jurisdiction => {
                            if jurisdiction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            jurisdiction__ = Some(map.next_value()?);
                        }
                        GeneratedField::Reason => {
                            if reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            reason__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgRetire {
                    owner: owner__.unwrap_or_default(),
                    credits: credits__.unwrap_or_default(),
                    jurisdiction: jurisdiction__.unwrap_or_default(),
                    reason: reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgRetire", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgRetireResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgRetireResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRetireResponse {
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
            type Value = MsgRetireResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgRetireResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgRetireResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRetireResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgRetireResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgSealBatch {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgSealBatch", len)?;
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSealBatch {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "issuer",
            "batch_denom",
            "batchDenom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Issuer,
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
                            "issuer" => Ok(GeneratedField::Issuer),
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
            type Value = MsgSealBatch;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgSealBatch")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgSealBatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut issuer__ = None;
                let mut batch_denom__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgSealBatch {
                    issuer: issuer__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgSealBatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgSealBatchResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgSealBatchResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSealBatchResponse {
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
            type Value = MsgSealBatchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgSealBatchResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgSealBatchResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSealBatchResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgSealBatchResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgSend {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.recipient.is_empty() {
            len += 1;
        }
        if !self.credits.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgSend", len)?;
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.credits.is_empty() {
            struct_ser.serialize_field("credits", &self.credits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSend {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "sender",
            "recipient",
            "credits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Recipient,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "recipient" => Ok(GeneratedField::Recipient),
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
            type Value = MsgSend;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgSend")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgSend, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut recipient__ = None;
                let mut credits__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map.next_value()?);
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map.next_value()?);
                        }
                        GeneratedField::Credits => {
                            if credits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credits"));
                            }
                            credits__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgSend {
                    sender: sender__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    credits: credits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgSend", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for msg_send::SendCredits {
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
        if !self.tradable_amount.is_empty() {
            len += 1;
        }
        if !self.retired_amount.is_empty() {
            len += 1;
        }
        if !self.retirement_jurisdiction.is_empty() {
            len += 1;
        }
        if !self.retirement_reason.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgSend.SendCredits", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.tradable_amount.is_empty() {
            struct_ser.serialize_field("tradableAmount", &self.tradable_amount)?;
        }
        if !self.retired_amount.is_empty() {
            struct_ser.serialize_field("retiredAmount", &self.retired_amount)?;
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
impl<'de> serde::Deserialize<'de> for msg_send::SendCredits {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_denom",
            "batchDenom",
            "tradable_amount",
            "tradableAmount",
            "retired_amount",
            "retiredAmount",
            "retirement_jurisdiction",
            "retirementJurisdiction",
            "retirement_reason",
            "retirementReason",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchDenom,
            TradableAmount,
            RetiredAmount,
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
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "tradableAmount" | "tradable_amount" => Ok(GeneratedField::TradableAmount),
                            "retiredAmount" | "retired_amount" => Ok(GeneratedField::RetiredAmount),
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
            type Value = msg_send::SendCredits;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgSend.SendCredits")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<msg_send::SendCredits, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                let mut tradable_amount__ = None;
                let mut retired_amount__ = None;
                let mut retirement_jurisdiction__ = None;
                let mut retirement_reason__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::TradableAmount => {
                            if tradable_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradableAmount"));
                            }
                            tradable_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetiredAmount => {
                            if retired_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retiredAmount"));
                            }
                            retired_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetirementJurisdiction => {
                            if retirement_jurisdiction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retirementJurisdiction"));
                            }
                            retirement_jurisdiction__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetirementReason => {
                            if retirement_reason__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retirementReason"));
                            }
                            retirement_reason__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(msg_send::SendCredits {
                    batch_denom: batch_denom__.unwrap_or_default(),
                    tradable_amount: tradable_amount__.unwrap_or_default(),
                    retired_amount: retired_amount__.unwrap_or_default(),
                    retirement_jurisdiction: retirement_jurisdiction__.unwrap_or_default(),
                    retirement_reason: retirement_reason__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgSend.SendCredits", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgSendResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgSendResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSendResponse {
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
            type Value = MsgSendResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgSendResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgSendResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSendResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgSendResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgSetClassCreatorAllowlist {
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
        if self.enabled {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgSetClassCreatorAllowlist", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetClassCreatorAllowlist {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "enabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Enabled,
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
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgSetClassCreatorAllowlist;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgSetClassCreatorAllowlist")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgSetClassCreatorAllowlist, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut enabled__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgSetClassCreatorAllowlist {
                    authority: authority__.unwrap_or_default(),
                    enabled: enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgSetClassCreatorAllowlist", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgSetClassCreatorAllowlistResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgSetClassCreatorAllowlistResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSetClassCreatorAllowlistResponse {
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
            type Value = MsgSetClassCreatorAllowlistResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgSetClassCreatorAllowlistResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgSetClassCreatorAllowlistResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSetClassCreatorAllowlistResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgSetClassCreatorAllowlistResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateBatchMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        if !self.new_metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateBatchMetadata", len)?;
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.new_metadata.is_empty() {
            struct_ser.serialize_field("newMetadata", &self.new_metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateBatchMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "issuer",
            "batch_denom",
            "batchDenom",
            "new_metadata",
            "newMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Issuer,
            BatchDenom,
            NewMetadata,
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
                            "issuer" => Ok(GeneratedField::Issuer),
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "newMetadata" | "new_metadata" => Ok(GeneratedField::NewMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateBatchMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateBatchMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateBatchMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut issuer__ = None;
                let mut batch_denom__ = None;
                let mut new_metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::NewMetadata => {
                            if new_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newMetadata"));
                            }
                            new_metadata__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateBatchMetadata {
                    issuer: issuer__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                    new_metadata: new_metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateBatchMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateBatchMetadataResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateBatchMetadataResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateBatchMetadataResponse {
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
            type Value = MsgUpdateBatchMetadataResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateBatchMetadataResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateBatchMetadataResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateBatchMetadataResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateBatchMetadataResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateClassAdmin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.class_id.is_empty() {
            len += 1;
        }
        if !self.new_admin.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateClassAdmin", len)?;
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if !self.new_admin.is_empty() {
            struct_ser.serialize_field("newAdmin", &self.new_admin)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateClassAdmin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "admin",
            "class_id",
            "classId",
            "new_admin",
            "newAdmin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Admin,
            ClassId,
            NewAdmin,
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
                            "admin" => Ok(GeneratedField::Admin),
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
                            "newAdmin" | "new_admin" => Ok(GeneratedField::NewAdmin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateClassAdmin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateClassAdmin")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateClassAdmin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut admin__ = None;
                let mut class_id__ = None;
                let mut new_admin__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::NewAdmin => {
                            if new_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newAdmin"));
                            }
                            new_admin__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateClassAdmin {
                    admin: admin__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                    new_admin: new_admin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateClassAdmin", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateClassAdminResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateClassAdminResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateClassAdminResponse {
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
            type Value = MsgUpdateClassAdminResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateClassAdminResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateClassAdminResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateClassAdminResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateClassAdminResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateClassFee {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateClassFee", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.fee.as_ref() {
            struct_ser.serialize_field("fee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateClassFee {
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
            type Value = MsgUpdateClassFee;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateClassFee")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateClassFee, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgUpdateClassFee {
                    authority: authority__.unwrap_or_default(),
                    fee: fee__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateClassFee", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateClassFeeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateClassFeeResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateClassFeeResponse {
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
            type Value = MsgUpdateClassFeeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateClassFeeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateClassFeeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateClassFeeResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateClassFeeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateClassIssuers {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.class_id.is_empty() {
            len += 1;
        }
        if !self.add_issuers.is_empty() {
            len += 1;
        }
        if !self.remove_issuers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateClassIssuers", len)?;
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if !self.add_issuers.is_empty() {
            struct_ser.serialize_field("addIssuers", &self.add_issuers)?;
        }
        if !self.remove_issuers.is_empty() {
            struct_ser.serialize_field("removeIssuers", &self.remove_issuers)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateClassIssuers {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "admin",
            "class_id",
            "classId",
            "add_issuers",
            "addIssuers",
            "remove_issuers",
            "removeIssuers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Admin,
            ClassId,
            AddIssuers,
            RemoveIssuers,
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
                            "admin" => Ok(GeneratedField::Admin),
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
                            "addIssuers" | "add_issuers" => Ok(GeneratedField::AddIssuers),
                            "removeIssuers" | "remove_issuers" => Ok(GeneratedField::RemoveIssuers),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateClassIssuers;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateClassIssuers")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateClassIssuers, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut admin__ = None;
                let mut class_id__ = None;
                let mut add_issuers__ = None;
                let mut remove_issuers__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::AddIssuers => {
                            if add_issuers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addIssuers"));
                            }
                            add_issuers__ = Some(map.next_value()?);
                        }
                        GeneratedField::RemoveIssuers => {
                            if remove_issuers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("removeIssuers"));
                            }
                            remove_issuers__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateClassIssuers {
                    admin: admin__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                    add_issuers: add_issuers__.unwrap_or_default(),
                    remove_issuers: remove_issuers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateClassIssuers", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateClassIssuersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateClassIssuersResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateClassIssuersResponse {
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
            type Value = MsgUpdateClassIssuersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateClassIssuersResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateClassIssuersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateClassIssuersResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateClassIssuersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateClassMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.class_id.is_empty() {
            len += 1;
        }
        if !self.new_metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateClassMetadata", len)?;
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if !self.new_metadata.is_empty() {
            struct_ser.serialize_field("newMetadata", &self.new_metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateClassMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "admin",
            "class_id",
            "classId",
            "new_metadata",
            "newMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Admin,
            ClassId,
            NewMetadata,
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
                            "admin" => Ok(GeneratedField::Admin),
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
                            "newMetadata" | "new_metadata" => Ok(GeneratedField::NewMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateClassMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateClassMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateClassMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut admin__ = None;
                let mut class_id__ = None;
                let mut new_metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::NewMetadata => {
                            if new_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newMetadata"));
                            }
                            new_metadata__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateClassMetadata {
                    admin: admin__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                    new_metadata: new_metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateClassMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateClassMetadataResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateClassMetadataResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateClassMetadataResponse {
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
            type Value = MsgUpdateClassMetadataResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateClassMetadataResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateClassMetadataResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateClassMetadataResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateClassMetadataResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateProjectAdmin {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.new_admin.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateProjectAdmin", len)?;
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.new_admin.is_empty() {
            struct_ser.serialize_field("newAdmin", &self.new_admin)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateProjectAdmin {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "admin",
            "project_id",
            "projectId",
            "new_admin",
            "newAdmin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Admin,
            ProjectId,
            NewAdmin,
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
                            "admin" => Ok(GeneratedField::Admin),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "newAdmin" | "new_admin" => Ok(GeneratedField::NewAdmin),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateProjectAdmin;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateProjectAdmin")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateProjectAdmin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut admin__ = None;
                let mut project_id__ = None;
                let mut new_admin__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::NewAdmin => {
                            if new_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newAdmin"));
                            }
                            new_admin__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateProjectAdmin {
                    admin: admin__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    new_admin: new_admin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateProjectAdmin", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateProjectAdminResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateProjectAdminResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateProjectAdminResponse {
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
            type Value = MsgUpdateProjectAdminResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateProjectAdminResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateProjectAdminResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateProjectAdminResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateProjectAdminResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateProjectEnrollment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.class_id.is_empty() {
            len += 1;
        }
        if self.new_status != 0 {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateProjectEnrollment", len)?;
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if self.new_status != 0 {
            let v = ProjectEnrollmentStatus::from_i32(self.new_status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.new_status)))?;
            struct_ser.serialize_field("newStatus", &v)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateProjectEnrollment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "issuer",
            "project_id",
            "projectId",
            "class_id",
            "classId",
            "new_status",
            "newStatus",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Issuer,
            ProjectId,
            ClassId,
            NewStatus,
            Metadata,
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
                            "issuer" => Ok(GeneratedField::Issuer),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
                            "newStatus" | "new_status" => Ok(GeneratedField::NewStatus),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateProjectEnrollment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateProjectEnrollment")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateProjectEnrollment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut issuer__ = None;
                let mut project_id__ = None;
                let mut class_id__ = None;
                let mut new_status__ = None;
                let mut metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::NewStatus => {
                            if new_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newStatus"));
                            }
                            new_status__ = Some(map.next_value::<ProjectEnrollmentStatus>()? as i32);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateProjectEnrollment {
                    issuer: issuer__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                    new_status: new_status__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateProjectEnrollment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateProjectEnrollmentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateProjectEnrollmentResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateProjectEnrollmentResponse {
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
            type Value = MsgUpdateProjectEnrollmentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateProjectEnrollmentResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateProjectEnrollmentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateProjectEnrollmentResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateProjectEnrollmentResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateProjectFee {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateProjectFee", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.fee.as_ref() {
            struct_ser.serialize_field("fee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateProjectFee {
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
            type Value = MsgUpdateProjectFee;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateProjectFee")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateProjectFee, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = map.next_value()?;
                        }
                    }
                }
                Ok(MsgUpdateProjectFee {
                    authority: authority__.unwrap_or_default(),
                    fee: fee__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateProjectFee", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateProjectFeeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateProjectFeeResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateProjectFeeResponse {
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
            type Value = MsgUpdateProjectFeeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateProjectFeeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateProjectFeeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateProjectFeeResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateProjectFeeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateProjectMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.new_metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateProjectMetadata", len)?;
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.new_metadata.is_empty() {
            struct_ser.serialize_field("newMetadata", &self.new_metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateProjectMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "admin",
            "project_id",
            "projectId",
            "new_metadata",
            "newMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Admin,
            ProjectId,
            NewMetadata,
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
                            "admin" => Ok(GeneratedField::Admin),
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            "newMetadata" | "new_metadata" => Ok(GeneratedField::NewMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateProjectMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateProjectMetadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateProjectMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut admin__ = None;
                let mut project_id__ = None;
                let mut new_metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::NewMetadata => {
                            if new_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newMetadata"));
                            }
                            new_metadata__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateProjectMetadata {
                    admin: admin__.unwrap_or_default(),
                    project_id: project_id__.unwrap_or_default(),
                    new_metadata: new_metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateProjectMetadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MsgUpdateProjectMetadataResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.MsgUpdateProjectMetadataResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateProjectMetadataResponse {
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
            type Value = MsgUpdateProjectMetadataResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.MsgUpdateProjectMetadataResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgUpdateProjectMetadataResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateProjectMetadataResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.MsgUpdateProjectMetadataResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OriginTx {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.source.is_empty() {
            len += 1;
        }
        if !self.contract.is_empty() {
            len += 1;
        }
        if !self.note.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.OriginTx", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.source.is_empty() {
            struct_ser.serialize_field("source", &self.source)?;
        }
        if !self.contract.is_empty() {
            struct_ser.serialize_field("contract", &self.contract)?;
        }
        if !self.note.is_empty() {
            struct_ser.serialize_field("note", &self.note)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OriginTx {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "source",
            "contract",
            "note",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Source,
            Contract,
            Note,
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
                            "source" => Ok(GeneratedField::Source),
                            "contract" => Ok(GeneratedField::Contract),
                            "note" => Ok(GeneratedField::Note),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OriginTx;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.OriginTx")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OriginTx, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut source__ = None;
                let mut contract__ = None;
                let mut note__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map.next_value()?);
                        }
                        GeneratedField::Contract => {
                            if contract__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contract"));
                            }
                            contract__ = Some(map.next_value()?);
                        }
                        GeneratedField::Note => {
                            if note__.is_some() {
                                return Err(serde::de::Error::duplicate_field("note"));
                            }
                            note__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OriginTx {
                    id: id__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                    contract: contract__.unwrap_or_default(),
                    note: note__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.OriginTx", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OriginTxIndex {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.class_key != 0 {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.source.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.OriginTxIndex", len)?;
        if self.class_key != 0 {
            struct_ser.serialize_field("classKey", ToString::to_string(&self.class_key).as_str())?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.source.is_empty() {
            struct_ser.serialize_field("source", &self.source)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OriginTxIndex {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_key",
            "classKey",
            "id",
            "source",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClassKey,
            Id,
            Source,
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
                            "classKey" | "class_key" => Ok(GeneratedField::ClassKey),
                            "id" => Ok(GeneratedField::Id),
                            "source" => Ok(GeneratedField::Source),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OriginTxIndex;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.OriginTxIndex")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<OriginTxIndex, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_key__ = None;
                let mut id__ = None;
                let mut source__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassKey => {
                            if class_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classKey"));
                            }
                            class_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Source => {
                            if source__.is_some() {
                                return Err(serde::de::Error::duplicate_field("source"));
                            }
                            source__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(OriginTxIndex {
                    class_key: class_key__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    source: source__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.OriginTxIndex", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Params {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.credit_class_fee.is_empty() {
            len += 1;
        }
        if !self.basket_fee.is_empty() {
            len += 1;
        }
        if !self.allowed_class_creators.is_empty() {
            len += 1;
        }
        if self.allowlist_enabled {
            len += 1;
        }
        if !self.allowed_denoms.is_empty() {
            len += 1;
        }
        if !self.allowed_bridge_chains.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.Params", len)?;
        if !self.credit_class_fee.is_empty() {
            struct_ser.serialize_field("creditClassFee", &self.credit_class_fee)?;
        }
        if !self.basket_fee.is_empty() {
            struct_ser.serialize_field("basketFee", &self.basket_fee)?;
        }
        if !self.allowed_class_creators.is_empty() {
            struct_ser.serialize_field("allowedClassCreators", &self.allowed_class_creators)?;
        }
        if self.allowlist_enabled {
            struct_ser.serialize_field("allowlistEnabled", &self.allowlist_enabled)?;
        }
        if !self.allowed_denoms.is_empty() {
            struct_ser.serialize_field("allowedDenoms", &self.allowed_denoms)?;
        }
        if !self.allowed_bridge_chains.is_empty() {
            struct_ser.serialize_field("allowedBridgeChains", &self.allowed_bridge_chains)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Params {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "credit_class_fee",
            "creditClassFee",
            "basket_fee",
            "basketFee",
            "allowed_class_creators",
            "allowedClassCreators",
            "allowlist_enabled",
            "allowlistEnabled",
            "allowed_denoms",
            "allowedDenoms",
            "allowed_bridge_chains",
            "allowedBridgeChains",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreditClassFee,
            BasketFee,
            AllowedClassCreators,
            AllowlistEnabled,
            AllowedDenoms,
            AllowedBridgeChains,
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
                            "creditClassFee" | "credit_class_fee" => Ok(GeneratedField::CreditClassFee),
                            "basketFee" | "basket_fee" => Ok(GeneratedField::BasketFee),
                            "allowedClassCreators" | "allowed_class_creators" => Ok(GeneratedField::AllowedClassCreators),
                            "allowlistEnabled" | "allowlist_enabled" => Ok(GeneratedField::AllowlistEnabled),
                            "allowedDenoms" | "allowed_denoms" => Ok(GeneratedField::AllowedDenoms),
                            "allowedBridgeChains" | "allowed_bridge_chains" => Ok(GeneratedField::AllowedBridgeChains),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Params;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.Params")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Params, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credit_class_fee__ = None;
                let mut basket_fee__ = None;
                let mut allowed_class_creators__ = None;
                let mut allowlist_enabled__ = None;
                let mut allowed_denoms__ = None;
                let mut allowed_bridge_chains__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CreditClassFee => {
                            if credit_class_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditClassFee"));
                            }
                            credit_class_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::BasketFee => {
                            if basket_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketFee"));
                            }
                            basket_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowedClassCreators => {
                            if allowed_class_creators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedClassCreators"));
                            }
                            allowed_class_creators__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowlistEnabled => {
                            if allowlist_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowlistEnabled"));
                            }
                            allowlist_enabled__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowedDenoms => {
                            if allowed_denoms__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedDenoms"));
                            }
                            allowed_denoms__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowedBridgeChains => {
                            if allowed_bridge_chains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedBridgeChains"));
                            }
                            allowed_bridge_chains__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    credit_class_fee: credit_class_fee__.unwrap_or_default(),
                    basket_fee: basket_fee__.unwrap_or_default(),
                    allowed_class_creators: allowed_class_creators__.unwrap_or_default(),
                    allowlist_enabled: allowlist_enabled__.unwrap_or_default(),
                    allowed_denoms: allowed_denoms__.unwrap_or_default(),
                    allowed_bridge_chains: allowed_bridge_chains__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.Params", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Project {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.key != 0 {
            len += 1;
        }
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if self.class_key != 0 {
            len += 1;
        }
        if !self.jurisdiction.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if !self.reference_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.Project", len)?;
        if self.key != 0 {
            struct_ser.serialize_field("key", ToString::to_string(&self.key).as_str())?;
        }
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", pbjson::private::base64::encode(&self.admin).as_str())?;
        }
        if self.class_key != 0 {
            struct_ser.serialize_field("classKey", ToString::to_string(&self.class_key).as_str())?;
        }
        if !self.jurisdiction.is_empty() {
            struct_ser.serialize_field("jurisdiction", &self.jurisdiction)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if !self.reference_id.is_empty() {
            struct_ser.serialize_field("referenceId", &self.reference_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Project {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "id",
            "admin",
            "class_key",
            "classKey",
            "jurisdiction",
            "metadata",
            "reference_id",
            "referenceId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Id,
            Admin,
            ClassKey,
            Jurisdiction,
            Metadata,
            ReferenceId,
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
                            "key" => Ok(GeneratedField::Key),
                            "id" => Ok(GeneratedField::Id),
                            "admin" => Ok(GeneratedField::Admin),
                            "classKey" | "class_key" => Ok(GeneratedField::ClassKey),
                            "jurisdiction" => Ok(GeneratedField::Jurisdiction),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "referenceId" | "reference_id" => Ok(GeneratedField::ReferenceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Project;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.Project")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Project, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut id__ = None;
                let mut admin__ = None;
                let mut class_key__ = None;
                let mut jurisdiction__ = None;
                let mut metadata__ = None;
                let mut reference_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ClassKey => {
                            if class_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classKey"));
                            }
                            class_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Jurisdiction => {
                            if jurisdiction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            jurisdiction__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReferenceId => {
                            if reference_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceId"));
                            }
                            reference_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Project {
                    key: key__.unwrap_or_default(),
                    id: id__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    class_key: class_key__.unwrap_or_default(),
                    jurisdiction: jurisdiction__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    reference_id: reference_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.Project", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectEnrollment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.project_key != 0 {
            len += 1;
        }
        if self.class_key != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if !self.application_metadata.is_empty() {
            len += 1;
        }
        if !self.enrollment_metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.ProjectEnrollment", len)?;
        if self.project_key != 0 {
            struct_ser.serialize_field("projectKey", ToString::to_string(&self.project_key).as_str())?;
        }
        if self.class_key != 0 {
            struct_ser.serialize_field("classKey", ToString::to_string(&self.class_key).as_str())?;
        }
        if self.status != 0 {
            let v = ProjectEnrollmentStatus::from_i32(self.status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.application_metadata.is_empty() {
            struct_ser.serialize_field("applicationMetadata", &self.application_metadata)?;
        }
        if !self.enrollment_metadata.is_empty() {
            struct_ser.serialize_field("enrollmentMetadata", &self.enrollment_metadata)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectEnrollment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_key",
            "projectKey",
            "class_key",
            "classKey",
            "status",
            "application_metadata",
            "applicationMetadata",
            "enrollment_metadata",
            "enrollmentMetadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectKey,
            ClassKey,
            Status,
            ApplicationMetadata,
            EnrollmentMetadata,
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
                            "projectKey" | "project_key" => Ok(GeneratedField::ProjectKey),
                            "classKey" | "class_key" => Ok(GeneratedField::ClassKey),
                            "status" => Ok(GeneratedField::Status),
                            "applicationMetadata" | "application_metadata" => Ok(GeneratedField::ApplicationMetadata),
                            "enrollmentMetadata" | "enrollment_metadata" => Ok(GeneratedField::EnrollmentMetadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectEnrollment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.ProjectEnrollment")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProjectEnrollment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_key__ = None;
                let mut class_key__ = None;
                let mut status__ = None;
                let mut application_metadata__ = None;
                let mut enrollment_metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectKey => {
                            if project_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectKey"));
                            }
                            project_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ClassKey => {
                            if class_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classKey"));
                            }
                            class_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value::<ProjectEnrollmentStatus>()? as i32);
                        }
                        GeneratedField::ApplicationMetadata => {
                            if application_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicationMetadata"));
                            }
                            application_metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnrollmentMetadata => {
                            if enrollment_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enrollmentMetadata"));
                            }
                            enrollment_metadata__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ProjectEnrollment {
                    project_key: project_key__.unwrap_or_default(),
                    class_key: class_key__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    application_metadata: application_metadata__.unwrap_or_default(),
                    enrollment_metadata: enrollment_metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.ProjectEnrollment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectEnrollmentStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "PROJECT_ENROLLMENT_STATUS_UNSPECIFIED",
            Self::Accepted => "PROJECT_ENROLLMENT_STATUS_ACCEPTED",
            Self::ChangesRequested => "PROJECT_ENROLLMENT_STATUS_CHANGES_REQUESTED",
            Self::Rejected => "PROJECT_ENROLLMENT_STATUS_REJECTED",
            Self::Terminated => "PROJECT_ENROLLMENT_STATUS_TERMINATED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ProjectEnrollmentStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "PROJECT_ENROLLMENT_STATUS_UNSPECIFIED",
            "PROJECT_ENROLLMENT_STATUS_ACCEPTED",
            "PROJECT_ENROLLMENT_STATUS_CHANGES_REQUESTED",
            "PROJECT_ENROLLMENT_STATUS_REJECTED",
            "PROJECT_ENROLLMENT_STATUS_TERMINATED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectEnrollmentStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ProjectEnrollmentStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ProjectEnrollmentStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "PROJECT_ENROLLMENT_STATUS_UNSPECIFIED" => Ok(ProjectEnrollmentStatus::Unspecified),
                    "PROJECT_ENROLLMENT_STATUS_ACCEPTED" => Ok(ProjectEnrollmentStatus::Accepted),
                    "PROJECT_ENROLLMENT_STATUS_CHANGES_REQUESTED" => Ok(ProjectEnrollmentStatus::ChangesRequested),
                    "PROJECT_ENROLLMENT_STATUS_REJECTED" => Ok(ProjectEnrollmentStatus::Rejected),
                    "PROJECT_ENROLLMENT_STATUS_TERMINATED" => Ok(ProjectEnrollmentStatus::Terminated),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectFee {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.ProjectFee", len)?;
        if let Some(v) = self.fee.as_ref() {
            struct_ser.serialize_field("fee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectFee {
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
            type Value = ProjectFee;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.ProjectFee")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProjectFee, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = map.next_value()?;
                        }
                    }
                }
                Ok(ProjectFee {
                    fee: fee__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.ProjectFee", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.class_id.is_empty() {
            len += 1;
        }
        if !self.jurisdiction.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if !self.reference_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.ProjectInfo", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if !self.jurisdiction.is_empty() {
            struct_ser.serialize_field("jurisdiction", &self.jurisdiction)?;
        }
        if !self.metadata.is_empty() {
            struct_ser.serialize_field("metadata", &self.metadata)?;
        }
        if !self.reference_id.is_empty() {
            struct_ser.serialize_field("referenceId", &self.reference_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "admin",
            "class_id",
            "classId",
            "jurisdiction",
            "metadata",
            "reference_id",
            "referenceId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Admin,
            ClassId,
            Jurisdiction,
            Metadata,
            ReferenceId,
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
                            "admin" => Ok(GeneratedField::Admin),
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
                            "jurisdiction" => Ok(GeneratedField::Jurisdiction),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "referenceId" | "reference_id" => Ok(GeneratedField::ReferenceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.ProjectInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProjectInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut admin__ = None;
                let mut class_id__ = None;
                let mut jurisdiction__ = None;
                let mut metadata__ = None;
                let mut reference_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Jurisdiction => {
                            if jurisdiction__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jurisdiction"));
                            }
                            jurisdiction__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                        GeneratedField::ReferenceId => {
                            if reference_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceId"));
                            }
                            reference_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ProjectInfo {
                    id: id__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                    jurisdiction: jurisdiction__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    reference_id: reference_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.ProjectInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ProjectSequence {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.class_key != 0 {
            len += 1;
        }
        if self.next_sequence != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.ProjectSequence", len)?;
        if self.class_key != 0 {
            struct_ser.serialize_field("classKey", ToString::to_string(&self.class_key).as_str())?;
        }
        if self.next_sequence != 0 {
            struct_ser.serialize_field("nextSequence", ToString::to_string(&self.next_sequence).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ProjectSequence {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_key",
            "classKey",
            "next_sequence",
            "nextSequence",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClassKey,
            NextSequence,
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
                            "classKey" | "class_key" => Ok(GeneratedField::ClassKey),
                            "nextSequence" | "next_sequence" => Ok(GeneratedField::NextSequence),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ProjectSequence;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.ProjectSequence")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ProjectSequence, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_key__ = None;
                let mut next_sequence__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassKey => {
                            if class_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classKey"));
                            }
                            class_key__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NextSequence => {
                            if next_sequence__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextSequence"));
                            }
                            next_sequence__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ProjectSequence {
                    class_key: class_key__.unwrap_or_default(),
                    next_sequence: next_sequence__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.ProjectSequence", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAllBalancesRequest {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryAllBalancesRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllBalancesRequest {
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
            type Value = QueryAllBalancesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryAllBalancesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryAllBalancesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryAllBalancesRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryAllBalancesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAllBalancesResponse {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryAllBalancesResponse", len)?;
        if !self.balances.is_empty() {
            struct_ser.serialize_field("balances", &self.balances)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllBalancesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "balances",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Balances,
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
                            "balances" => Ok(GeneratedField::Balances),
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
            type Value = QueryAllBalancesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryAllBalancesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryAllBalancesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut balances__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Balances => {
                            if balances__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balances"));
                            }
                            balances__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryAllBalancesResponse {
                    balances: balances__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryAllBalancesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAllowedBridgeChainsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryAllowedBridgeChainsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllowedBridgeChainsRequest {
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
            type Value = QueryAllowedBridgeChainsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryAllowedBridgeChainsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryAllowedBridgeChainsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryAllowedBridgeChainsRequest {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryAllowedBridgeChainsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAllowedBridgeChainsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.allowed_bridge_chains.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryAllowedBridgeChainsResponse", len)?;
        if !self.allowed_bridge_chains.is_empty() {
            struct_ser.serialize_field("allowedBridgeChains", &self.allowed_bridge_chains)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllowedBridgeChainsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "allowed_bridge_chains",
            "allowedBridgeChains",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowedBridgeChains,
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
                            "allowedBridgeChains" | "allowed_bridge_chains" => Ok(GeneratedField::AllowedBridgeChains),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllowedBridgeChainsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryAllowedBridgeChainsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryAllowedBridgeChainsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut allowed_bridge_chains__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AllowedBridgeChains => {
                            if allowed_bridge_chains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedBridgeChains"));
                            }
                            allowed_bridge_chains__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryAllowedBridgeChainsResponse {
                    allowed_bridge_chains: allowed_bridge_chains__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryAllowedBridgeChainsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAllowedClassCreatorsRequest {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryAllowedClassCreatorsRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllowedClassCreatorsRequest {
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
            type Value = QueryAllowedClassCreatorsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryAllowedClassCreatorsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryAllowedClassCreatorsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryAllowedClassCreatorsRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryAllowedClassCreatorsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryAllowedClassCreatorsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.class_creators.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryAllowedClassCreatorsResponse", len)?;
        if !self.class_creators.is_empty() {
            struct_ser.serialize_field("classCreators", &self.class_creators)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllowedClassCreatorsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_creators",
            "classCreators",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClassCreators,
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
                            "classCreators" | "class_creators" => Ok(GeneratedField::ClassCreators),
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
            type Value = QueryAllowedClassCreatorsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryAllowedClassCreatorsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryAllowedClassCreatorsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_creators__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassCreators => {
                            if class_creators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classCreators"));
                            }
                            class_creators__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryAllowedClassCreatorsResponse {
                    class_creators: class_creators__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryAllowedClassCreatorsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBalanceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBalanceRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBalanceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "batch_denom",
            "batchDenom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            "address" => Ok(GeneratedField::Address),
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
            type Value = QueryBalanceRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBalanceRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBalanceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut batch_denom__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryBalanceRequest {
                    address: address__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBalanceRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBalanceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.balance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBalanceResponse", len)?;
        if let Some(v) = self.balance.as_ref() {
            struct_ser.serialize_field("balance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBalanceResponse {
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
            type Value = QueryBalanceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBalanceResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBalanceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut balance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBalanceResponse {
                    balance: balance__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBalanceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBalancesByBatchRequest {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBalancesByBatchRequest", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBalancesByBatchRequest {
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
            type Value = QueryBalancesByBatchRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBalancesByBatchRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBalancesByBatchRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBalancesByBatchRequest {
                    batch_denom: batch_denom__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBalancesByBatchRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBalancesByBatchResponse {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBalancesByBatchResponse", len)?;
        if !self.balances.is_empty() {
            struct_ser.serialize_field("balances", &self.balances)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBalancesByBatchResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "balances",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Balances,
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
                            "balances" => Ok(GeneratedField::Balances),
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
            type Value = QueryBalancesByBatchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBalancesByBatchResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBalancesByBatchResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut balances__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Balances => {
                            if balances__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balances"));
                            }
                            balances__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBalancesByBatchResponse {
                    balances: balances__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBalancesByBatchResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBalancesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBalancesRequest", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBalancesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
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
                            "address" => Ok(GeneratedField::Address),
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
            type Value = QueryBalancesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBalancesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBalancesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBalancesRequest {
                    address: address__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBalancesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBalancesResponse {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBalancesResponse", len)?;
        if !self.balances.is_empty() {
            struct_ser.serialize_field("balances", &self.balances)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBalancesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "balances",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Balances,
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
                            "balances" => Ok(GeneratedField::Balances),
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
            type Value = QueryBalancesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBalancesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBalancesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut balances__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Balances => {
                            if balances__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balances"));
                            }
                            balances__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBalancesResponse {
                    balances: balances__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBalancesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBatchRequest {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBatchRequest", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBatchRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_denom",
            "batchDenom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = QueryBatchRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBatchRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBatchRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryBatchRequest {
                    batch_denom: batch_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBatchRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBatchResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.batch.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBatchResponse", len)?;
        if let Some(v) = self.batch.as_ref() {
            struct_ser.serialize_field("batch", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBatchResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Batch,
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
                            "batch" => Ok(GeneratedField::Batch),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBatchResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBatchResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBatchResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Batch => {
                            if batch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batch"));
                            }
                            batch__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBatchResponse {
                    batch: batch__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBatchResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBatchesByClassRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.class_id.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBatchesByClassRequest", len)?;
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBatchesByClassRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_id",
            "classId",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClassId,
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
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
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
            type Value = QueryBatchesByClassRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBatchesByClassRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBatchesByClassRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBatchesByClassRequest {
                    class_id: class_id__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBatchesByClassRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBatchesByClassResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.batches.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBatchesByClassResponse", len)?;
        if !self.batches.is_empty() {
            struct_ser.serialize_field("batches", &self.batches)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBatchesByClassResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batches",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Batches,
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
                            "batches" => Ok(GeneratedField::Batches),
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
            type Value = QueryBatchesByClassResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBatchesByClassResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBatchesByClassResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batches__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Batches => {
                            if batches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batches"));
                            }
                            batches__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBatchesByClassResponse {
                    batches: batches__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBatchesByClassResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBatchesByIssuerRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.issuer.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBatchesByIssuerRequest", len)?;
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBatchesByIssuerRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "issuer",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Issuer,
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
                            "issuer" => Ok(GeneratedField::Issuer),
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
            type Value = QueryBatchesByIssuerRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBatchesByIssuerRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBatchesByIssuerRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut issuer__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBatchesByIssuerRequest {
                    issuer: issuer__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBatchesByIssuerRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBatchesByIssuerResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.batches.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBatchesByIssuerResponse", len)?;
        if !self.batches.is_empty() {
            struct_ser.serialize_field("batches", &self.batches)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBatchesByIssuerResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batches",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Batches,
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
                            "batches" => Ok(GeneratedField::Batches),
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
            type Value = QueryBatchesByIssuerResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBatchesByIssuerResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBatchesByIssuerResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batches__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Batches => {
                            if batches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batches"));
                            }
                            batches__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBatchesByIssuerResponse {
                    batches: batches__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBatchesByIssuerResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBatchesByProjectRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBatchesByProjectRequest", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBatchesByProjectRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
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
            type Value = QueryBatchesByProjectRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBatchesByProjectRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBatchesByProjectRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBatchesByProjectRequest {
                    project_id: project_id__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBatchesByProjectRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBatchesByProjectResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.batches.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBatchesByProjectResponse", len)?;
        if !self.batches.is_empty() {
            struct_ser.serialize_field("batches", &self.batches)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBatchesByProjectResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batches",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Batches,
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
                            "batches" => Ok(GeneratedField::Batches),
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
            type Value = QueryBatchesByProjectResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBatchesByProjectResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBatchesByProjectResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batches__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Batches => {
                            if batches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batches"));
                            }
                            batches__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBatchesByProjectResponse {
                    batches: batches__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBatchesByProjectResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBatchesRequest {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBatchesRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBatchesRequest {
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
            type Value = QueryBatchesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBatchesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBatchesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBatchesRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBatchesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBatchesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.batches.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryBatchesResponse", len)?;
        if !self.batches.is_empty() {
            struct_ser.serialize_field("batches", &self.batches)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBatchesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batches",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Batches,
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
                            "batches" => Ok(GeneratedField::Batches),
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
            type Value = QueryBatchesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryBatchesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryBatchesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batches__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Batches => {
                            if batches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batches"));
                            }
                            batches__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryBatchesResponse {
                    batches: batches__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryBatchesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassCreatorAllowlistRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryClassCreatorAllowlistRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassCreatorAllowlistRequest {
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
            type Value = QueryClassCreatorAllowlistRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryClassCreatorAllowlistRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryClassCreatorAllowlistRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryClassCreatorAllowlistRequest {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryClassCreatorAllowlistRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassCreatorAllowlistResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.enabled {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryClassCreatorAllowlistResponse", len)?;
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassCreatorAllowlistResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enabled",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enabled,
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
                            "enabled" => Ok(GeneratedField::Enabled),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryClassCreatorAllowlistResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryClassCreatorAllowlistResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryClassCreatorAllowlistResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enabled__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryClassCreatorAllowlistResponse {
                    enabled: enabled__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryClassCreatorAllowlistResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassFeeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryClassFeeRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassFeeRequest {
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
            type Value = QueryClassFeeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryClassFeeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryClassFeeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryClassFeeRequest {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryClassFeeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassFeeResponse {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryClassFeeResponse", len)?;
        if let Some(v) = self.fee.as_ref() {
            struct_ser.serialize_field("fee", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassFeeResponse {
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
            type Value = QueryClassFeeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryClassFeeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryClassFeeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Fee => {
                            if fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fee"));
                            }
                            fee__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryClassFeeResponse {
                    fee: fee__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryClassFeeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassIssuersRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.class_id.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryClassIssuersRequest", len)?;
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassIssuersRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_id",
            "classId",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClassId,
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
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
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
            type Value = QueryClassIssuersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryClassIssuersRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryClassIssuersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryClassIssuersRequest {
                    class_id: class_id__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryClassIssuersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassIssuersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.issuers.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryClassIssuersResponse", len)?;
        if !self.issuers.is_empty() {
            struct_ser.serialize_field("issuers", &self.issuers)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassIssuersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "issuers",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Issuers,
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
                            "issuers" => Ok(GeneratedField::Issuers),
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
            type Value = QueryClassIssuersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryClassIssuersResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryClassIssuersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut issuers__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Issuers => {
                            if issuers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuers"));
                            }
                            issuers__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryClassIssuersResponse {
                    issuers: issuers__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryClassIssuersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.class_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryClassRequest", len)?;
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_id",
            "classId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = QueryClassRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryClassRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryClassRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryClassRequest {
                    class_id: class_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryClassRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.class.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryClassResponse", len)?;
        if let Some(v) = self.class.as_ref() {
            struct_ser.serialize_field("class", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Class,
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
                            "class" => Ok(GeneratedField::Class),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryClassResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryClassResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryClassResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Class => {
                            if class__.is_some() {
                                return Err(serde::de::Error::duplicate_field("class"));
                            }
                            class__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryClassResponse {
                    class: class__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryClassResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassesByAdminRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.admin.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryClassesByAdminRequest", len)?;
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassesByAdminRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "admin",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Admin,
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
                            "admin" => Ok(GeneratedField::Admin),
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
            type Value = QueryClassesByAdminRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryClassesByAdminRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryClassesByAdminRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut admin__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryClassesByAdminRequest {
                    admin: admin__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryClassesByAdminRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassesByAdminResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.classes.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryClassesByAdminResponse", len)?;
        if !self.classes.is_empty() {
            struct_ser.serialize_field("classes", &self.classes)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassesByAdminResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "classes",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Classes,
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
                            "classes" => Ok(GeneratedField::Classes),
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
            type Value = QueryClassesByAdminResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryClassesByAdminResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryClassesByAdminResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut classes__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Classes => {
                            if classes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classes"));
                            }
                            classes__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryClassesByAdminResponse {
                    classes: classes__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryClassesByAdminResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassesRequest {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryClassesRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassesRequest {
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
            type Value = QueryClassesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryClassesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryClassesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryClassesRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryClassesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.classes.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryClassesResponse", len)?;
        if !self.classes.is_empty() {
            struct_ser.serialize_field("classes", &self.classes)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "classes",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Classes,
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
                            "classes" => Ok(GeneratedField::Classes),
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
            type Value = QueryClassesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryClassesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryClassesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut classes__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Classes => {
                            if classes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classes"));
                            }
                            classes__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryClassesResponse {
                    classes: classes__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryClassesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryCreditTypeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.abbreviation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryCreditTypeRequest", len)?;
        if !self.abbreviation.is_empty() {
            struct_ser.serialize_field("abbreviation", &self.abbreviation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCreditTypeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "abbreviation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Abbreviation,
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
                            "abbreviation" => Ok(GeneratedField::Abbreviation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCreditTypeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryCreditTypeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryCreditTypeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut abbreviation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Abbreviation => {
                            if abbreviation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abbreviation"));
                            }
                            abbreviation__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryCreditTypeRequest {
                    abbreviation: abbreviation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryCreditTypeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryCreditTypeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.credit_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryCreditTypeResponse", len)?;
        if let Some(v) = self.credit_type.as_ref() {
            struct_ser.serialize_field("creditType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCreditTypeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "credit_type",
            "creditType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreditType,
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
                            "creditType" | "credit_type" => Ok(GeneratedField::CreditType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCreditTypeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryCreditTypeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryCreditTypeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credit_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CreditType => {
                            if credit_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditType"));
                            }
                            credit_type__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryCreditTypeResponse {
                    credit_type: credit_type__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryCreditTypeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryCreditTypesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryCreditTypesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCreditTypesRequest {
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
            type Value = QueryCreditTypesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryCreditTypesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryCreditTypesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryCreditTypesRequest {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryCreditTypesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryCreditTypesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.credit_types.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryCreditTypesResponse", len)?;
        if !self.credit_types.is_empty() {
            struct_ser.serialize_field("creditTypes", &self.credit_types)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCreditTypesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "credit_types",
            "creditTypes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreditTypes,
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
                            "creditTypes" | "credit_types" => Ok(GeneratedField::CreditTypes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCreditTypesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryCreditTypesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryCreditTypesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credit_types__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CreditTypes => {
                            if credit_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditTypes"));
                            }
                            credit_types__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryCreditTypesResponse {
                    credit_types: credit_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryCreditTypesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryParamsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryParamsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsRequest {
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
            type Value = QueryParamsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryParamsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryParamsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryParamsRequest {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryParamsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryParamsResponse", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "params",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
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
                            "params" => Ok(GeneratedField::Params),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryParamsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryParamsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryParamsResponse {
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryParamsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryProjectEnrollmentRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        if !self.class_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryProjectEnrollmentRequest", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProjectEnrollmentRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "class_id",
            "classId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
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
            type Value = QueryProjectEnrollmentRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryProjectEnrollmentRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProjectEnrollmentRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut class_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryProjectEnrollmentRequest {
                    project_id: project_id__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryProjectEnrollmentRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryProjectEnrollmentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.project_class.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryProjectEnrollmentResponse", len)?;
        if let Some(v) = self.project_class.as_ref() {
            struct_ser.serialize_field("projectClass", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProjectEnrollmentResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_class",
            "projectClass",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectClass,
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
                            "projectClass" | "project_class" => Ok(GeneratedField::ProjectClass),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryProjectEnrollmentResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryProjectEnrollmentResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProjectEnrollmentResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_class__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectClass => {
                            if project_class__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectClass"));
                            }
                            project_class__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryProjectEnrollmentResponse {
                    project_class: project_class__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryProjectEnrollmentResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryProjectEnrollmentsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryProjectEnrollmentsRequest", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProjectEnrollmentsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
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
            type Value = QueryProjectEnrollmentsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryProjectEnrollmentsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProjectEnrollmentsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryProjectEnrollmentsRequest {
                    project_id: project_id__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryProjectEnrollmentsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryProjectEnrollmentsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.enrollments.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryProjectEnrollmentsResponse", len)?;
        if !self.enrollments.is_empty() {
            struct_ser.serialize_field("enrollments", &self.enrollments)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProjectEnrollmentsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "enrollments",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Enrollments,
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
                            "enrollments" => Ok(GeneratedField::Enrollments),
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
            type Value = QueryProjectEnrollmentsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryProjectEnrollmentsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProjectEnrollmentsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut enrollments__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Enrollments => {
                            if enrollments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enrollments"));
                            }
                            enrollments__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryProjectEnrollmentsResponse {
                    enrollments: enrollments__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryProjectEnrollmentsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryProjectRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.project_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryProjectRequest", len)?;
        if !self.project_id.is_empty() {
            struct_ser.serialize_field("projectId", &self.project_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProjectRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project_id",
            "projectId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ProjectId,
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
                            "projectId" | "project_id" => Ok(GeneratedField::ProjectId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryProjectRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryProjectRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProjectRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ProjectId => {
                            if project_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectId"));
                            }
                            project_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryProjectRequest {
                    project_id: project_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryProjectRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryProjectResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.project.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryProjectResponse", len)?;
        if let Some(v) = self.project.as_ref() {
            struct_ser.serialize_field("project", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProjectResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "project",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Project,
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
                            "project" => Ok(GeneratedField::Project),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryProjectResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryProjectResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProjectResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut project__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Project => {
                            if project__.is_some() {
                                return Err(serde::de::Error::duplicate_field("project"));
                            }
                            project__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryProjectResponse {
                    project: project__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryProjectResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryProjectsByAdminRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.admin.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryProjectsByAdminRequest", len)?;
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProjectsByAdminRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "admin",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Admin,
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
                            "admin" => Ok(GeneratedField::Admin),
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
            type Value = QueryProjectsByAdminRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryProjectsByAdminRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProjectsByAdminRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut admin__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryProjectsByAdminRequest {
                    admin: admin__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryProjectsByAdminRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryProjectsByAdminResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.projects.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryProjectsByAdminResponse", len)?;
        if !self.projects.is_empty() {
            struct_ser.serialize_field("projects", &self.projects)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProjectsByAdminResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "projects",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Projects,
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
                            "projects" => Ok(GeneratedField::Projects),
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
            type Value = QueryProjectsByAdminResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryProjectsByAdminResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProjectsByAdminResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut projects__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Projects => {
                            if projects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projects"));
                            }
                            projects__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryProjectsByAdminResponse {
                    projects: projects__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryProjectsByAdminResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryProjectsByClassRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.class_id.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryProjectsByClassRequest", len)?;
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProjectsByClassRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "class_id",
            "classId",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClassId,
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
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
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
            type Value = QueryProjectsByClassRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryProjectsByClassRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProjectsByClassRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryProjectsByClassRequest {
                    class_id: class_id__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryProjectsByClassRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryProjectsByClassResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.projects.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryProjectsByClassResponse", len)?;
        if !self.projects.is_empty() {
            struct_ser.serialize_field("projects", &self.projects)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProjectsByClassResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "projects",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Projects,
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
                            "projects" => Ok(GeneratedField::Projects),
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
            type Value = QueryProjectsByClassResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryProjectsByClassResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProjectsByClassResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut projects__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Projects => {
                            if projects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projects"));
                            }
                            projects__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryProjectsByClassResponse {
                    projects: projects__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryProjectsByClassResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryProjectsByReferenceIdRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reference_id.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryProjectsByReferenceIdRequest", len)?;
        if !self.reference_id.is_empty() {
            struct_ser.serialize_field("referenceId", &self.reference_id)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProjectsByReferenceIdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reference_id",
            "referenceId",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReferenceId,
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
                            "referenceId" | "reference_id" => Ok(GeneratedField::ReferenceId),
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
            type Value = QueryProjectsByReferenceIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryProjectsByReferenceIdRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProjectsByReferenceIdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reference_id__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ReferenceId => {
                            if reference_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceId"));
                            }
                            reference_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryProjectsByReferenceIdRequest {
                    reference_id: reference_id__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryProjectsByReferenceIdRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryProjectsByReferenceIdResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.projects.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryProjectsByReferenceIdResponse", len)?;
        if !self.projects.is_empty() {
            struct_ser.serialize_field("projects", &self.projects)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProjectsByReferenceIdResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "projects",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Projects,
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
                            "projects" => Ok(GeneratedField::Projects),
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
            type Value = QueryProjectsByReferenceIdResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryProjectsByReferenceIdResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProjectsByReferenceIdResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut projects__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Projects => {
                            if projects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projects"));
                            }
                            projects__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryProjectsByReferenceIdResponse {
                    projects: projects__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryProjectsByReferenceIdResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryProjectsRequest {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryProjectsRequest", len)?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProjectsRequest {
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
            type Value = QueryProjectsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryProjectsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProjectsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryProjectsRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryProjectsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryProjectsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.projects.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QueryProjectsResponse", len)?;
        if !self.projects.is_empty() {
            struct_ser.serialize_field("projects", &self.projects)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryProjectsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "projects",
            "pagination",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Projects,
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
                            "projects" => Ok(GeneratedField::Projects),
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
            type Value = QueryProjectsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QueryProjectsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QueryProjectsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut projects__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Projects => {
                            if projects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projects"));
                            }
                            projects__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryProjectsResponse {
                    projects: projects__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QueryProjectsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuerySupplyRequest {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QuerySupplyRequest", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuerySupplyRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_denom",
            "batchDenom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = QuerySupplyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QuerySupplyRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuerySupplyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QuerySupplyRequest {
                    batch_denom: batch_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QuerySupplyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuerySupplyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.tradable_amount.is_empty() {
            len += 1;
        }
        if !self.retired_amount.is_empty() {
            len += 1;
        }
        if !self.cancelled_amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1.QuerySupplyResponse", len)?;
        if !self.tradable_amount.is_empty() {
            struct_ser.serialize_field("tradableAmount", &self.tradable_amount)?;
        }
        if !self.retired_amount.is_empty() {
            struct_ser.serialize_field("retiredAmount", &self.retired_amount)?;
        }
        if !self.cancelled_amount.is_empty() {
            struct_ser.serialize_field("cancelledAmount", &self.cancelled_amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuerySupplyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tradable_amount",
            "tradableAmount",
            "retired_amount",
            "retiredAmount",
            "cancelled_amount",
            "cancelledAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TradableAmount,
            RetiredAmount,
            CancelledAmount,
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
                            "tradableAmount" | "tradable_amount" => Ok(GeneratedField::TradableAmount),
                            "retiredAmount" | "retired_amount" => Ok(GeneratedField::RetiredAmount),
                            "cancelledAmount" | "cancelled_amount" => Ok(GeneratedField::CancelledAmount),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuerySupplyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1.QuerySupplyResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuerySupplyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tradable_amount__ = None;
                let mut retired_amount__ = None;
                let mut cancelled_amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TradableAmount => {
                            if tradable_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradableAmount"));
                            }
                            tradable_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::RetiredAmount => {
                            if retired_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retiredAmount"));
                            }
                            retired_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::CancelledAmount => {
                            if cancelled_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cancelledAmount"));
                            }
                            cancelled_amount__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QuerySupplyResponse {
                    tradable_amount: tradable_amount__.unwrap_or_default(),
                    retired_amount: retired_amount__.unwrap_or_default(),
                    cancelled_amount: cancelled_amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1.QuerySupplyResponse", FIELDS, GeneratedVisitor)
    }
}
