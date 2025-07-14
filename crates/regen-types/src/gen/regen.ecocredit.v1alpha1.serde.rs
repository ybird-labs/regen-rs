// @generated
impl serde::Serialize for Balance {
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
        if !self.tradable_balance.is_empty() {
            len += 1;
        }
        if !self.retired_balance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.Balance", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.tradable_balance.is_empty() {
            struct_ser.serialize_field("tradableBalance", &self.tradable_balance)?;
        }
        if !self.retired_balance.is_empty() {
            struct_ser.serialize_field("retiredBalance", &self.retired_balance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Balance {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "batch_denom",
            "batchDenom",
            "tradable_balance",
            "tradableBalance",
            "retired_balance",
            "retiredBalance",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            BatchDenom,
            TradableBalance,
            RetiredBalance,
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
                            "tradableBalance" | "tradable_balance" => Ok(GeneratedField::TradableBalance),
                            "retiredBalance" | "retired_balance" => Ok(GeneratedField::RetiredBalance),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Balance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.Balance")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Balance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut batch_denom__ = None;
                let mut tradable_balance__ = None;
                let mut retired_balance__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TradableBalance => {
                            if tradable_balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradableBalance"));
                            }
                            tradable_balance__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetiredBalance => {
                            if retired_balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retiredBalance"));
                            }
                            retired_balance__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Balance {
                    address: address__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                    tradable_balance: tradable_balance__.unwrap_or_default(),
                    retired_balance: retired_balance__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.Balance", FIELDS, GeneratedVisitor)
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
        if !self.class_id.is_empty() {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.total_amount.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if !self.amount_cancelled.is_empty() {
            len += 1;
        }
        if self.start_date.is_some() {
            len += 1;
        }
        if self.end_date.is_some() {
            len += 1;
        }
        if !self.project_location.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.BatchInfo", len)?;
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.total_amount.is_empty() {
            struct_ser.serialize_field("totalAmount", &self.total_amount)?;
        }
        if !self.metadata.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("metadata", pbjson::private::base64::encode(&self.metadata).as_str())?;
        }
        if !self.amount_cancelled.is_empty() {
            struct_ser.serialize_field("amountCancelled", &self.amount_cancelled)?;
        }
        if let Some(v) = self.start_date.as_ref() {
            struct_ser.serialize_field("startDate", v)?;
        }
        if let Some(v) = self.end_date.as_ref() {
            struct_ser.serialize_field("endDate", v)?;
        }
        if !self.project_location.is_empty() {
            struct_ser.serialize_field("projectLocation", &self.project_location)?;
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
            "class_id",
            "classId",
            "batch_denom",
            "batchDenom",
            "issuer",
            "total_amount",
            "totalAmount",
            "metadata",
            "amount_cancelled",
            "amountCancelled",
            "start_date",
            "startDate",
            "end_date",
            "endDate",
            "project_location",
            "projectLocation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClassId,
            BatchDenom,
            Issuer,
            TotalAmount,
            Metadata,
            AmountCancelled,
            StartDate,
            EndDate,
            ProjectLocation,
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
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "issuer" => Ok(GeneratedField::Issuer),
                            "totalAmount" | "total_amount" => Ok(GeneratedField::TotalAmount),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "amountCancelled" | "amount_cancelled" => Ok(GeneratedField::AmountCancelled),
                            "startDate" | "start_date" => Ok(GeneratedField::StartDate),
                            "endDate" | "end_date" => Ok(GeneratedField::EndDate),
                            "projectLocation" | "project_location" => Ok(GeneratedField::ProjectLocation),
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.BatchInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                let mut batch_denom__ = None;
                let mut issuer__ = None;
                let mut total_amount__ = None;
                let mut metadata__ = None;
                let mut amount_cancelled__ = None;
                let mut start_date__ = None;
                let mut end_date__ = None;
                let mut project_location__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalAmount => {
                            if total_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalAmount"));
                            }
                            total_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AmountCancelled => {
                            if amount_cancelled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amountCancelled"));
                            }
                            amount_cancelled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartDate => {
                            if start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startDate"));
                            }
                            start_date__ = map_.next_value()?;
                        }
                        GeneratedField::EndDate => {
                            if end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endDate"));
                            }
                            end_date__ = map_.next_value()?;
                        }
                        GeneratedField::ProjectLocation => {
                            if project_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectLocation"));
                            }
                            project_location__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchInfo {
                    class_id: class_id__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                    issuer: issuer__.unwrap_or_default(),
                    total_amount: total_amount__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    amount_cancelled: amount_cancelled__.unwrap_or_default(),
                    start_date: start_date__,
                    end_date: end_date__,
                    project_location: project_location__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.BatchInfo", FIELDS, GeneratedVisitor)
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
        if !self.class_id.is_empty() {
            len += 1;
        }
        if !self.admin.is_empty() {
            len += 1;
        }
        if !self.issuers.is_empty() {
            len += 1;
        }
        if !self.metadata.is_empty() {
            len += 1;
        }
        if self.credit_type.is_some() {
            len += 1;
        }
        if self.num_batches != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.ClassInfo", len)?;
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.issuers.is_empty() {
            struct_ser.serialize_field("issuers", &self.issuers)?;
        }
        if !self.metadata.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("metadata", pbjson::private::base64::encode(&self.metadata).as_str())?;
        }
        if let Some(v) = self.credit_type.as_ref() {
            struct_ser.serialize_field("creditType", v)?;
        }
        if self.num_batches != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("numBatches", ToString::to_string(&self.num_batches).as_str())?;
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
            "class_id",
            "classId",
            "admin",
            "issuers",
            "metadata",
            "credit_type",
            "creditType",
            "num_batches",
            "numBatches",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClassId,
            Admin,
            Issuers,
            Metadata,
            CreditType,
            NumBatches,
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
                            "admin" => Ok(GeneratedField::Admin),
                            "issuers" => Ok(GeneratedField::Issuers),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "creditType" | "credit_type" => Ok(GeneratedField::CreditType),
                            "numBatches" | "num_batches" => Ok(GeneratedField::NumBatches),
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.ClassInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClassInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                let mut admin__ = None;
                let mut issuers__ = None;
                let mut metadata__ = None;
                let mut credit_type__ = None;
                let mut num_batches__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Issuers => {
                            if issuers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuers"));
                            }
                            issuers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CreditType => {
                            if credit_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditType"));
                            }
                            credit_type__ = map_.next_value()?;
                        }
                        GeneratedField::NumBatches => {
                            if num_batches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numBatches"));
                            }
                            num_batches__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ClassInfo {
                    class_id: class_id__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                    issuers: issuers__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    credit_type: credit_type__,
                    num_batches: num_batches__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.ClassInfo", FIELDS, GeneratedVisitor)
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
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.abbreviation.is_empty() {
            len += 1;
        }
        if !self.unit.is_empty() {
            len += 1;
        }
        if self.precision != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.CreditType", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.abbreviation.is_empty() {
            struct_ser.serialize_field("abbreviation", &self.abbreviation)?;
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
            "name",
            "abbreviation",
            "unit",
            "precision",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Abbreviation,
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
                            "name" => Ok(GeneratedField::Name),
                            "abbreviation" => Ok(GeneratedField::Abbreviation),
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.CreditType")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreditType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut abbreviation__ = None;
                let mut unit__ = None;
                let mut precision__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Abbreviation => {
                            if abbreviation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abbreviation"));
                            }
                            abbreviation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Unit => {
                            if unit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unit"));
                            }
                            unit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Precision => {
                            if precision__.is_some() {
                                return Err(serde::de::Error::duplicate_field("precision"));
                            }
                            precision__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CreditType {
                    name: name__.unwrap_or_default(),
                    abbreviation: abbreviation__.unwrap_or_default(),
                    unit: unit__.unwrap_or_default(),
                    precision: precision__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.CreditType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreditTypeSeq {
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
        if self.seq_number != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.CreditTypeSeq", len)?;
        if !self.abbreviation.is_empty() {
            struct_ser.serialize_field("abbreviation", &self.abbreviation)?;
        }
        if self.seq_number != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("seqNumber", ToString::to_string(&self.seq_number).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreditTypeSeq {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "abbreviation",
            "seq_number",
            "seqNumber",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Abbreviation,
            SeqNumber,
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
                            "seqNumber" | "seq_number" => Ok(GeneratedField::SeqNumber),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreditTypeSeq;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.CreditTypeSeq")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreditTypeSeq, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut abbreviation__ = None;
                let mut seq_number__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Abbreviation => {
                            if abbreviation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abbreviation"));
                            }
                            abbreviation__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SeqNumber => {
                            if seq_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("seqNumber"));
                            }
                            seq_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CreditTypeSeq {
                    abbreviation: abbreviation__.unwrap_or_default(),
                    seq_number: seq_number__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.CreditTypeSeq", FIELDS, GeneratedVisitor)
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
        if !self.canceller.is_empty() {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.EventCancel", len)?;
        if !self.canceller.is_empty() {
            struct_ser.serialize_field("canceller", &self.canceller)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
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
            "canceller",
            "batch_denom",
            "batchDenom",
            "amount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Canceller,
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
                            "canceller" => Ok(GeneratedField::Canceller),
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
            type Value = EventCancel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.EventCancel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventCancel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut canceller__ = None;
                let mut batch_denom__ = None;
                let mut amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Canceller => {
                            if canceller__.is_some() {
                                return Err(serde::de::Error::duplicate_field("canceller"));
                            }
                            canceller__ = Some(map_.next_value()?);
                        }
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
                Ok(EventCancel {
                    canceller: canceller__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.EventCancel", FIELDS, GeneratedVisitor)
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
        if !self.class_id.is_empty() {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        if !self.issuer.is_empty() {
            len += 1;
        }
        if !self.total_amount.is_empty() {
            len += 1;
        }
        if !self.start_date.is_empty() {
            len += 1;
        }
        if !self.end_date.is_empty() {
            len += 1;
        }
        if !self.project_location.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.EventCreateBatch", len)?;
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.total_amount.is_empty() {
            struct_ser.serialize_field("totalAmount", &self.total_amount)?;
        }
        if !self.start_date.is_empty() {
            struct_ser.serialize_field("startDate", &self.start_date)?;
        }
        if !self.end_date.is_empty() {
            struct_ser.serialize_field("endDate", &self.end_date)?;
        }
        if !self.project_location.is_empty() {
            struct_ser.serialize_field("projectLocation", &self.project_location)?;
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
            "class_id",
            "classId",
            "batch_denom",
            "batchDenom",
            "issuer",
            "total_amount",
            "totalAmount",
            "start_date",
            "startDate",
            "end_date",
            "endDate",
            "project_location",
            "projectLocation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClassId,
            BatchDenom,
            Issuer,
            TotalAmount,
            StartDate,
            EndDate,
            ProjectLocation,
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
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "issuer" => Ok(GeneratedField::Issuer),
                            "totalAmount" | "total_amount" => Ok(GeneratedField::TotalAmount),
                            "startDate" | "start_date" => Ok(GeneratedField::StartDate),
                            "endDate" | "end_date" => Ok(GeneratedField::EndDate),
                            "projectLocation" | "project_location" => Ok(GeneratedField::ProjectLocation),
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.EventCreateBatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventCreateBatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                let mut batch_denom__ = None;
                let mut issuer__ = None;
                let mut total_amount__ = None;
                let mut start_date__ = None;
                let mut end_date__ = None;
                let mut project_location__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TotalAmount => {
                            if total_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalAmount"));
                            }
                            total_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StartDate => {
                            if start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startDate"));
                            }
                            start_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EndDate => {
                            if end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endDate"));
                            }
                            end_date__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ProjectLocation => {
                            if project_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectLocation"));
                            }
                            project_location__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventCreateBatch {
                    class_id: class_id__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                    issuer: issuer__.unwrap_or_default(),
                    total_amount: total_amount__.unwrap_or_default(),
                    start_date: start_date__.unwrap_or_default(),
                    end_date: end_date__.unwrap_or_default(),
                    project_location: project_location__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.EventCreateBatch", FIELDS, GeneratedVisitor)
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
        if !self.admin.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.EventCreateClass", len)?;
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
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
            "admin",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClassId,
            Admin,
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
                            "admin" => Ok(GeneratedField::Admin),
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.EventCreateClass")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventCreateClass, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                let mut admin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventCreateClass {
                    class_id: class_id__.unwrap_or_default(),
                    admin: admin__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.EventCreateClass", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventReceive {
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
        if !self.basket_denom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.EventReceive", len)?;
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
        if !self.basket_denom.is_empty() {
            struct_ser.serialize_field("basketDenom", &self.basket_denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EventReceive {
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
            "basket_denom",
            "basketDenom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Sender,
            Recipient,
            BatchDenom,
            TradableAmount,
            RetiredAmount,
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
                            "sender" => Ok(GeneratedField::Sender),
                            "recipient" => Ok(GeneratedField::Recipient),
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "tradableAmount" | "tradable_amount" => Ok(GeneratedField::TradableAmount),
                            "retiredAmount" | "retired_amount" => Ok(GeneratedField::RetiredAmount),
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
            type Value = EventReceive;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.EventReceive")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventReceive, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut recipient__ = None;
                let mut batch_denom__ = None;
                let mut tradable_amount__ = None;
                let mut retired_amount__ = None;
                let mut basket_denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TradableAmount => {
                            if tradable_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradableAmount"));
                            }
                            tradable_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetiredAmount => {
                            if retired_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retiredAmount"));
                            }
                            retired_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BasketDenom => {
                            if basket_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketDenom"));
                            }
                            basket_denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventReceive {
                    sender: sender__.unwrap_or_default(),
                    recipient: recipient__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                    tradable_amount: tradable_amount__.unwrap_or_default(),
                    retired_amount: retired_amount__.unwrap_or_default(),
                    basket_denom: basket_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.EventReceive", FIELDS, GeneratedVisitor)
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
        if !self.retirer.is_empty() {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        if !self.amount.is_empty() {
            len += 1;
        }
        if !self.location.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.EventRetire", len)?;
        if !self.retirer.is_empty() {
            struct_ser.serialize_field("retirer", &self.retirer)?;
        }
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
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
            "retirer",
            "batch_denom",
            "batchDenom",
            "amount",
            "location",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Retirer,
            BatchDenom,
            Amount,
            Location,
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
                            "retirer" => Ok(GeneratedField::Retirer),
                            "batchDenom" | "batch_denom" => Ok(GeneratedField::BatchDenom),
                            "amount" => Ok(GeneratedField::Amount),
                            "location" => Ok(GeneratedField::Location),
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.EventRetire")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EventRetire, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut retirer__ = None;
                let mut batch_denom__ = None;
                let mut amount__ = None;
                let mut location__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Retirer => {
                            if retirer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retirer"));
                            }
                            retirer__ = Some(map_.next_value()?);
                        }
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
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(EventRetire {
                    retirer: retirer__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                    location: location__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.EventRetire", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GenesisState {
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
        if !self.class_info.is_empty() {
            len += 1;
        }
        if !self.batch_info.is_empty() {
            len += 1;
        }
        if !self.sequences.is_empty() {
            len += 1;
        }
        if !self.balances.is_empty() {
            len += 1;
        }
        if !self.supplies.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.class_info.is_empty() {
            struct_ser.serialize_field("classInfo", &self.class_info)?;
        }
        if !self.batch_info.is_empty() {
            struct_ser.serialize_field("batchInfo", &self.batch_info)?;
        }
        if !self.sequences.is_empty() {
            struct_ser.serialize_field("sequences", &self.sequences)?;
        }
        if !self.balances.is_empty() {
            struct_ser.serialize_field("balances", &self.balances)?;
        }
        if !self.supplies.is_empty() {
            struct_ser.serialize_field("supplies", &self.supplies)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenesisState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "params",
            "class_info",
            "classInfo",
            "batch_info",
            "batchInfo",
            "sequences",
            "balances",
            "supplies",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            ClassInfo,
            BatchInfo,
            Sequences,
            Balances,
            Supplies,
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
                            "classInfo" | "class_info" => Ok(GeneratedField::ClassInfo),
                            "batchInfo" | "batch_info" => Ok(GeneratedField::BatchInfo),
                            "sequences" => Ok(GeneratedField::Sequences),
                            "balances" => Ok(GeneratedField::Balances),
                            "supplies" => Ok(GeneratedField::Supplies),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenesisState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.GenesisState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GenesisState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut class_info__ = None;
                let mut batch_info__ = None;
                let mut sequences__ = None;
                let mut balances__ = None;
                let mut supplies__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                        GeneratedField::ClassInfo => {
                            if class_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classInfo"));
                            }
                            class_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BatchInfo => {
                            if batch_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchInfo"));
                            }
                            batch_info__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Sequences => {
                            if sequences__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequences"));
                            }
                            sequences__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Balances => {
                            if balances__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balances"));
                            }
                            balances__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Supplies => {
                            if supplies__.is_some() {
                                return Err(serde::de::Error::duplicate_field("supplies"));
                            }
                            supplies__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    class_info: class_info__.unwrap_or_default(),
                    batch_info: batch_info__.unwrap_or_default(),
                    sequences: sequences__.unwrap_or_default(),
                    balances: balances__.unwrap_or_default(),
                    supplies: supplies__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.GenesisState", FIELDS, GeneratedVisitor)
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
        if !self.holder.is_empty() {
            len += 1;
        }
        if !self.credits.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgCancel", len)?;
        if !self.holder.is_empty() {
            struct_ser.serialize_field("holder", &self.holder)?;
        }
        if !self.credits.is_empty() {
            struct_ser.serialize_field("credits", &self.credits)?;
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
            "holder",
            "credits",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Holder,
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
                            "holder" => Ok(GeneratedField::Holder),
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
            type Value = MsgCancel;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgCancel")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgCancel, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut holder__ = None;
                let mut credits__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Holder => {
                            if holder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("holder"));
                            }
                            holder__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Credits => {
                            if credits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credits"));
                            }
                            credits__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgCancel {
                    holder: holder__.unwrap_or_default(),
                    credits: credits__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgCancel", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for msg_cancel::CancelCredits {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgCancel.CancelCredits", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for msg_cancel::CancelCredits {
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
            type Value = msg_cancel::CancelCredits;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgCancel.CancelCredits")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<msg_cancel::CancelCredits, V::Error>
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
                Ok(msg_cancel::CancelCredits {
                    batch_denom: batch_denom__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgCancel.CancelCredits", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgCancelResponse", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgCancelResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgCancelResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCancelResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgCancelResponse", FIELDS, GeneratedVisitor)
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
        if !self.class_id.is_empty() {
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
        if !self.project_location.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgCreateBatch", len)?;
        if !self.issuer.is_empty() {
            struct_ser.serialize_field("issuer", &self.issuer)?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if !self.issuance.is_empty() {
            struct_ser.serialize_field("issuance", &self.issuance)?;
        }
        if !self.metadata.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("metadata", pbjson::private::base64::encode(&self.metadata).as_str())?;
        }
        if let Some(v) = self.start_date.as_ref() {
            struct_ser.serialize_field("startDate", v)?;
        }
        if let Some(v) = self.end_date.as_ref() {
            struct_ser.serialize_field("endDate", v)?;
        }
        if !self.project_location.is_empty() {
            struct_ser.serialize_field("projectLocation", &self.project_location)?;
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
            "class_id",
            "classId",
            "issuance",
            "metadata",
            "start_date",
            "startDate",
            "end_date",
            "endDate",
            "project_location",
            "projectLocation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Issuer,
            ClassId,
            Issuance,
            Metadata,
            StartDate,
            EndDate,
            ProjectLocation,
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
                            "issuance" => Ok(GeneratedField::Issuance),
                            "metadata" => Ok(GeneratedField::Metadata),
                            "startDate" | "start_date" => Ok(GeneratedField::StartDate),
                            "endDate" | "end_date" => Ok(GeneratedField::EndDate),
                            "projectLocation" | "project_location" => Ok(GeneratedField::ProjectLocation),
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgCreateBatch")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgCreateBatch, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut issuer__ = None;
                let mut class_id__ = None;
                let mut issuance__ = None;
                let mut metadata__ = None;
                let mut start_date__ = None;
                let mut end_date__ = None;
                let mut project_location__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Issuer => {
                            if issuer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuer"));
                            }
                            issuer__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Issuance => {
                            if issuance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuance"));
                            }
                            issuance__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StartDate => {
                            if start_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startDate"));
                            }
                            start_date__ = map_.next_value()?;
                        }
                        GeneratedField::EndDate => {
                            if end_date__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endDate"));
                            }
                            end_date__ = map_.next_value()?;
                        }
                        GeneratedField::ProjectLocation => {
                            if project_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("projectLocation"));
                            }
                            project_location__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgCreateBatch {
                    issuer: issuer__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                    issuance: issuance__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    start_date: start_date__,
                    end_date: end_date__,
                    project_location: project_location__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgCreateBatch", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for msg_create_batch::BatchIssuance {
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
        if !self.retirement_location.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgCreateBatch.BatchIssuance", len)?;
        if !self.recipient.is_empty() {
            struct_ser.serialize_field("recipient", &self.recipient)?;
        }
        if !self.tradable_amount.is_empty() {
            struct_ser.serialize_field("tradableAmount", &self.tradable_amount)?;
        }
        if !self.retired_amount.is_empty() {
            struct_ser.serialize_field("retiredAmount", &self.retired_amount)?;
        }
        if !self.retirement_location.is_empty() {
            struct_ser.serialize_field("retirementLocation", &self.retirement_location)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for msg_create_batch::BatchIssuance {
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
            "retirement_location",
            "retirementLocation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Recipient,
            TradableAmount,
            RetiredAmount,
            RetirementLocation,
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
                            "retirementLocation" | "retirement_location" => Ok(GeneratedField::RetirementLocation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = msg_create_batch::BatchIssuance;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgCreateBatch.BatchIssuance")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<msg_create_batch::BatchIssuance, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut recipient__ = None;
                let mut tradable_amount__ = None;
                let mut retired_amount__ = None;
                let mut retirement_location__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TradableAmount => {
                            if tradable_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradableAmount"));
                            }
                            tradable_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetiredAmount => {
                            if retired_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retiredAmount"));
                            }
                            retired_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetirementLocation => {
                            if retirement_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retirementLocation"));
                            }
                            retirement_location__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(msg_create_batch::BatchIssuance {
                    recipient: recipient__.unwrap_or_default(),
                    tradable_amount: tradable_amount__.unwrap_or_default(),
                    retired_amount: retired_amount__.unwrap_or_default(),
                    retirement_location: retirement_location__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgCreateBatch.BatchIssuance", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgCreateBatchResponse", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgCreateBatchResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgCreateBatchResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgCreateBatchResponse {
                    batch_denom: batch_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgCreateBatchResponse", FIELDS, GeneratedVisitor)
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
        if !self.credit_type_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgCreateClass", len)?;
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.issuers.is_empty() {
            struct_ser.serialize_field("issuers", &self.issuers)?;
        }
        if !self.metadata.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("metadata", pbjson::private::base64::encode(&self.metadata).as_str())?;
        }
        if !self.credit_type_name.is_empty() {
            struct_ser.serialize_field("creditTypeName", &self.credit_type_name)?;
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
            "credit_type_name",
            "creditTypeName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Admin,
            Issuers,
            Metadata,
            CreditTypeName,
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
                            "creditTypeName" | "credit_type_name" => Ok(GeneratedField::CreditTypeName),
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgCreateClass")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgCreateClass, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut admin__ = None;
                let mut issuers__ = None;
                let mut metadata__ = None;
                let mut credit_type_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Issuers => {
                            if issuers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuers"));
                            }
                            issuers__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CreditTypeName => {
                            if credit_type_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditTypeName"));
                            }
                            credit_type_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgCreateClass {
                    admin: admin__.unwrap_or_default(),
                    issuers: issuers__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                    credit_type_name: credit_type_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgCreateClass", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgCreateClassResponse", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgCreateClassResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgCreateClassResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgCreateClassResponse {
                    class_id: class_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgCreateClassResponse", FIELDS, GeneratedVisitor)
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
        if !self.holder.is_empty() {
            len += 1;
        }
        if !self.credits.is_empty() {
            len += 1;
        }
        if !self.location.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgRetire", len)?;
        if !self.holder.is_empty() {
            struct_ser.serialize_field("holder", &self.holder)?;
        }
        if !self.credits.is_empty() {
            struct_ser.serialize_field("credits", &self.credits)?;
        }
        if !self.location.is_empty() {
            struct_ser.serialize_field("location", &self.location)?;
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
            "holder",
            "credits",
            "location",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Holder,
            Credits,
            Location,
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
                            "holder" => Ok(GeneratedField::Holder),
                            "credits" => Ok(GeneratedField::Credits),
                            "location" => Ok(GeneratedField::Location),
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgRetire")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgRetire, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut holder__ = None;
                let mut credits__ = None;
                let mut location__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Holder => {
                            if holder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("holder"));
                            }
                            holder__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Credits => {
                            if credits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credits"));
                            }
                            credits__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Location => {
                            if location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            location__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgRetire {
                    holder: holder__.unwrap_or_default(),
                    credits: credits__.unwrap_or_default(),
                    location: location__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgRetire", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for msg_retire::RetireCredits {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgRetire.RetireCredits", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.amount.is_empty() {
            struct_ser.serialize_field("amount", &self.amount)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for msg_retire::RetireCredits {
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
            type Value = msg_retire::RetireCredits;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgRetire.RetireCredits")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<msg_retire::RetireCredits, V::Error>
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
                Ok(msg_retire::RetireCredits {
                    batch_denom: batch_denom__.unwrap_or_default(),
                    amount: amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgRetire.RetireCredits", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgRetireResponse", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgRetireResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgRetireResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRetireResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgRetireResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgSend", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgSend")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgSend, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut sender__ = None;
                let mut recipient__ = None;
                let mut credits__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Recipient => {
                            if recipient__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recipient"));
                            }
                            recipient__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Credits => {
                            if credits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("credits"));
                            }
                            credits__ = Some(map_.next_value()?);
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
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgSend", FIELDS, GeneratedVisitor)
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
        if !self.retirement_location.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgSend.SendCredits", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.tradable_amount.is_empty() {
            struct_ser.serialize_field("tradableAmount", &self.tradable_amount)?;
        }
        if !self.retired_amount.is_empty() {
            struct_ser.serialize_field("retiredAmount", &self.retired_amount)?;
        }
        if !self.retirement_location.is_empty() {
            struct_ser.serialize_field("retirementLocation", &self.retirement_location)?;
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
            "retirement_location",
            "retirementLocation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchDenom,
            TradableAmount,
            RetiredAmount,
            RetirementLocation,
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
                            "retirementLocation" | "retirement_location" => Ok(GeneratedField::RetirementLocation),
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgSend.SendCredits")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<msg_send::SendCredits, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                let mut tradable_amount__ = None;
                let mut retired_amount__ = None;
                let mut retirement_location__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TradableAmount => {
                            if tradable_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradableAmount"));
                            }
                            tradable_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetiredAmount => {
                            if retired_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retiredAmount"));
                            }
                            retired_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetirementLocation => {
                            if retirement_location__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retirementLocation"));
                            }
                            retirement_location__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(msg_send::SendCredits {
                    batch_denom: batch_denom__.unwrap_or_default(),
                    tradable_amount: tradable_amount__.unwrap_or_default(),
                    retired_amount: retired_amount__.unwrap_or_default(),
                    retirement_location: retirement_location__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgSend.SendCredits", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgSendResponse", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgSendResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgSendResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSendResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgSendResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgUpdateClassAdmin", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgUpdateClassAdmin")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateClassAdmin, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut admin__ = None;
                let mut class_id__ = None;
                let mut new_admin__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NewAdmin => {
                            if new_admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newAdmin"));
                            }
                            new_admin__ = Some(map_.next_value()?);
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
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgUpdateClassAdmin", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgUpdateClassAdminResponse", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgUpdateClassAdminResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateClassAdminResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateClassAdminResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgUpdateClassAdminResponse", FIELDS, GeneratedVisitor)
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
        if !self.issuers.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgUpdateClassIssuers", len)?;
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if !self.issuers.is_empty() {
            struct_ser.serialize_field("issuers", &self.issuers)?;
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
            "issuers",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Admin,
            ClassId,
            Issuers,
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
                            "issuers" => Ok(GeneratedField::Issuers),
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgUpdateClassIssuers")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateClassIssuers, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut admin__ = None;
                let mut class_id__ = None;
                let mut issuers__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Issuers => {
                            if issuers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("issuers"));
                            }
                            issuers__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MsgUpdateClassIssuers {
                    admin: admin__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                    issuers: issuers__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgUpdateClassIssuers", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgUpdateClassIssuersResponse", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgUpdateClassIssuersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateClassIssuersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateClassIssuersResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgUpdateClassIssuersResponse", FIELDS, GeneratedVisitor)
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
        if !self.metadata.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgUpdateClassMetadata", len)?;
        if !self.admin.is_empty() {
            struct_ser.serialize_field("admin", &self.admin)?;
        }
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        if !self.metadata.is_empty() {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("metadata", pbjson::private::base64::encode(&self.metadata).as_str())?;
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
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Admin,
            ClassId,
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
                            "admin" => Ok(GeneratedField::Admin),
                            "classId" | "class_id" => Ok(GeneratedField::ClassId),
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
            type Value = MsgUpdateClassMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgUpdateClassMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateClassMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut admin__ = None;
                let mut class_id__ = None;
                let mut metadata__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Admin => {
                            if admin__.is_some() {
                                return Err(serde::de::Error::duplicate_field("admin"));
                            }
                            admin__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(MsgUpdateClassMetadata {
                    admin: admin__.unwrap_or_default(),
                    class_id: class_id__.unwrap_or_default(),
                    metadata: metadata__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgUpdateClassMetadata", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.MsgUpdateClassMetadataResponse", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.MsgUpdateClassMetadataResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MsgUpdateClassMetadataResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateClassMetadataResponse {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.MsgUpdateClassMetadataResponse", FIELDS, GeneratedVisitor)
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
        if !self.allowed_class_creators.is_empty() {
            len += 1;
        }
        if self.allowlist_enabled {
            len += 1;
        }
        if !self.credit_types.is_empty() {
            len += 1;
        }
        if !self.basket_creation_fee.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.Params", len)?;
        if !self.credit_class_fee.is_empty() {
            struct_ser.serialize_field("creditClassFee", &self.credit_class_fee)?;
        }
        if !self.allowed_class_creators.is_empty() {
            struct_ser.serialize_field("allowedClassCreators", &self.allowed_class_creators)?;
        }
        if self.allowlist_enabled {
            struct_ser.serialize_field("allowlistEnabled", &self.allowlist_enabled)?;
        }
        if !self.credit_types.is_empty() {
            struct_ser.serialize_field("creditTypes", &self.credit_types)?;
        }
        if !self.basket_creation_fee.is_empty() {
            struct_ser.serialize_field("basketCreationFee", &self.basket_creation_fee)?;
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
            "allowed_class_creators",
            "allowedClassCreators",
            "allowlist_enabled",
            "allowlistEnabled",
            "credit_types",
            "creditTypes",
            "basket_creation_fee",
            "basketCreationFee",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreditClassFee,
            AllowedClassCreators,
            AllowlistEnabled,
            CreditTypes,
            BasketCreationFee,
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
                            "allowedClassCreators" | "allowed_class_creators" => Ok(GeneratedField::AllowedClassCreators),
                            "allowlistEnabled" | "allowlist_enabled" => Ok(GeneratedField::AllowlistEnabled),
                            "creditTypes" | "credit_types" => Ok(GeneratedField::CreditTypes),
                            "basketCreationFee" | "basket_creation_fee" => Ok(GeneratedField::BasketCreationFee),
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.Params")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Params, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credit_class_fee__ = None;
                let mut allowed_class_creators__ = None;
                let mut allowlist_enabled__ = None;
                let mut credit_types__ = None;
                let mut basket_creation_fee__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CreditClassFee => {
                            if credit_class_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditClassFee"));
                            }
                            credit_class_fee__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowedClassCreators => {
                            if allowed_class_creators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowedClassCreators"));
                            }
                            allowed_class_creators__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AllowlistEnabled => {
                            if allowlist_enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowlistEnabled"));
                            }
                            allowlist_enabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreditTypes => {
                            if credit_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditTypes"));
                            }
                            credit_types__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BasketCreationFee => {
                            if basket_creation_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("basketCreationFee"));
                            }
                            basket_creation_fee__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Params {
                    credit_class_fee: credit_class_fee__.unwrap_or_default(),
                    allowed_class_creators: allowed_class_creators__.unwrap_or_default(),
                    allowlist_enabled: allowlist_enabled__.unwrap_or_default(),
                    credit_types: credit_types__.unwrap_or_default(),
                    basket_creation_fee: basket_creation_fee__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.Params", FIELDS, GeneratedVisitor)
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
        if !self.account.is_empty() {
            len += 1;
        }
        if !self.batch_denom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QueryBalanceRequest", len)?;
        if !self.account.is_empty() {
            struct_ser.serialize_field("account", &self.account)?;
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
            "account",
            "batch_denom",
            "batchDenom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
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
                            "account" => Ok(GeneratedField::Account),
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.QueryBalanceRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBalanceRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                let mut batch_denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBalanceRequest {
                    account: account__.unwrap_or_default(),
                    batch_denom: batch_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QueryBalanceRequest", FIELDS, GeneratedVisitor)
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
        if !self.tradable_amount.is_empty() {
            len += 1;
        }
        if !self.retired_amount.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QueryBalanceResponse", len)?;
        if !self.tradable_amount.is_empty() {
            struct_ser.serialize_field("tradableAmount", &self.tradable_amount)?;
        }
        if !self.retired_amount.is_empty() {
            struct_ser.serialize_field("retiredAmount", &self.retired_amount)?;
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
            "tradable_amount",
            "tradableAmount",
            "retired_amount",
            "retiredAmount",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = QueryBalanceResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.QueryBalanceResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBalanceResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tradable_amount__ = None;
                let mut retired_amount__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TradableAmount => {
                            if tradable_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradableAmount"));
                            }
                            tradable_amount__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetiredAmount => {
                            if retired_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retiredAmount"));
                            }
                            retired_amount__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBalanceResponse {
                    tradable_amount: tradable_amount__.unwrap_or_default(),
                    retired_amount: retired_amount__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QueryBalanceResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBatchInfoRequest {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QueryBatchInfoRequest", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBatchInfoRequest {
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
            type Value = QueryBatchInfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.QueryBatchInfoRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBatchInfoRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryBatchInfoRequest {
                    batch_denom: batch_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QueryBatchInfoRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryBatchInfoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QueryBatchInfoResponse", len)?;
        if let Some(v) = self.info.as_ref() {
            struct_ser.serialize_field("info", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryBatchInfoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "info",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Info,
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
                            "info" => Ok(GeneratedField::Info),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryBatchInfoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.QueryBatchInfoResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBatchInfoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Info => {
                            if info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryBatchInfoResponse {
                    info: info__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QueryBatchInfoResponse", FIELDS, GeneratedVisitor)
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
        if !self.class_id.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QueryBatchesRequest", len)?;
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
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
            type Value = QueryBatchesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.QueryBatchesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBatchesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryBatchesRequest {
                    class_id: class_id__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QueryBatchesRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QueryBatchesResponse", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.QueryBatchesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryBatchesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batches__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Batches => {
                            if batches__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batches"));
                            }
                            batches__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryBatchesResponse {
                    batches: batches__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QueryBatchesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassInfoRequest {
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QueryClassInfoRequest", len)?;
        if !self.class_id.is_empty() {
            struct_ser.serialize_field("classId", &self.class_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassInfoRequest {
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
            type Value = QueryClassInfoRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.QueryClassInfoRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryClassInfoRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut class_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClassId => {
                            if class_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classId"));
                            }
                            class_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryClassInfoRequest {
                    class_id: class_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QueryClassInfoRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QueryClassInfoResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.info.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QueryClassInfoResponse", len)?;
        if let Some(v) = self.info.as_ref() {
            struct_ser.serialize_field("info", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryClassInfoResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "info",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Info,
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
                            "info" => Ok(GeneratedField::Info),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryClassInfoResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.QueryClassInfoResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryClassInfoResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut info__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Info => {
                            if info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("info"));
                            }
                            info__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryClassInfoResponse {
                    info: info__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QueryClassInfoResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QueryClassesRequest", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.QueryClassesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryClassesRequest, V::Error>
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
                Ok(QueryClassesRequest {
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QueryClassesRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QueryClassesResponse", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.QueryClassesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryClassesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut classes__ = None;
                let mut pagination__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Classes => {
                            if classes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("classes"));
                            }
                            classes__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryClassesResponse {
                    classes: classes__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QueryClassesResponse", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QueryCreditTypesRequest", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.QueryCreditTypesRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryCreditTypesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryCreditTypesRequest {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QueryCreditTypesRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QueryCreditTypesResponse", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.QueryCreditTypesResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryCreditTypesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut credit_types__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::CreditTypes => {
                            if credit_types__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creditTypes"));
                            }
                            credit_types__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QueryCreditTypesResponse {
                    credit_types: credit_types__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QueryCreditTypesResponse", FIELDS, GeneratedVisitor)
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
        let struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QueryParamsRequest", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.QueryParamsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryParamsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryParamsRequest {
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QueryParamsRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QueryParamsResponse", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.QueryParamsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QueryParamsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map_.next_value()?;
                        }
                    }
                }
                Ok(QueryParamsResponse {
                    params: params__,
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QueryParamsResponse", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QuerySupplyRequest", len)?;
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.QuerySupplyRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuerySupplyRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QuerySupplyRequest {
                    batch_denom: batch_denom__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QuerySupplyRequest", FIELDS, GeneratedVisitor)
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
        if !self.tradable_supply.is_empty() {
            len += 1;
        }
        if !self.retired_supply.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.QuerySupplyResponse", len)?;
        if !self.tradable_supply.is_empty() {
            struct_ser.serialize_field("tradableSupply", &self.tradable_supply)?;
        }
        if !self.retired_supply.is_empty() {
            struct_ser.serialize_field("retiredSupply", &self.retired_supply)?;
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
            "tradable_supply",
            "tradableSupply",
            "retired_supply",
            "retiredSupply",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TradableSupply,
            RetiredSupply,
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
                            "tradableSupply" | "tradable_supply" => Ok(GeneratedField::TradableSupply),
                            "retiredSupply" | "retired_supply" => Ok(GeneratedField::RetiredSupply),
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
                formatter.write_str("struct regen.ecocredit.v1alpha1.QuerySupplyResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuerySupplyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut tradable_supply__ = None;
                let mut retired_supply__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::TradableSupply => {
                            if tradable_supply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradableSupply"));
                            }
                            tradable_supply__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetiredSupply => {
                            if retired_supply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retiredSupply"));
                            }
                            retired_supply__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QuerySupplyResponse {
                    tradable_supply: tradable_supply__.unwrap_or_default(),
                    retired_supply: retired_supply__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.QuerySupplyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Supply {
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
        if !self.tradable_supply.is_empty() {
            len += 1;
        }
        if !self.retired_supply.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("regen.ecocredit.v1alpha1.Supply", len)?;
        if !self.batch_denom.is_empty() {
            struct_ser.serialize_field("batchDenom", &self.batch_denom)?;
        }
        if !self.tradable_supply.is_empty() {
            struct_ser.serialize_field("tradableSupply", &self.tradable_supply)?;
        }
        if !self.retired_supply.is_empty() {
            struct_ser.serialize_field("retiredSupply", &self.retired_supply)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Supply {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "batch_denom",
            "batchDenom",
            "tradable_supply",
            "tradableSupply",
            "retired_supply",
            "retiredSupply",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BatchDenom,
            TradableSupply,
            RetiredSupply,
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
                            "tradableSupply" | "tradable_supply" => Ok(GeneratedField::TradableSupply),
                            "retiredSupply" | "retired_supply" => Ok(GeneratedField::RetiredSupply),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Supply;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct regen.ecocredit.v1alpha1.Supply")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Supply, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut batch_denom__ = None;
                let mut tradable_supply__ = None;
                let mut retired_supply__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BatchDenom => {
                            if batch_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("batchDenom"));
                            }
                            batch_denom__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TradableSupply => {
                            if tradable_supply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tradableSupply"));
                            }
                            tradable_supply__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetiredSupply => {
                            if retired_supply__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retiredSupply"));
                            }
                            retired_supply__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Supply {
                    batch_denom: batch_denom__.unwrap_or_default(),
                    tradable_supply: tradable_supply__.unwrap_or_default(),
                    retired_supply: retired_supply__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("regen.ecocredit.v1alpha1.Supply", FIELDS, GeneratedVisitor)
    }
}
