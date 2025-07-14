// @generated
/// EventAnchor is an event emitted when data is anchored on chain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAnchor {
    /// iri is the IRI of the data anchored on chain.
    #[prost(string, tag="1")]
    pub iri: ::prost::alloc::string::String,
}
impl ::prost::Name for EventAnchor {
const NAME: &'static str = "EventAnchor";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// EventAttest is an event emitted when data is attested to on chain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAttest {
    /// iri is the IRI of the data attested to.
    #[prost(string, tag="1")]
    pub iri: ::prost::alloc::string::String,
    /// attestor is the address of the account that has attested to the veracity of
    /// the data.
    #[prost(string, tag="2")]
    pub attestor: ::prost::alloc::string::String,
}
impl ::prost::Name for EventAttest {
const NAME: &'static str = "EventAttest";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// EventDefineResolver is an event emitted when a resolved is defined on chain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDefineResolver {
    /// id is the ID of the defined resolver.
    #[prost(uint64, tag="1")]
    pub id: u64,
}
impl ::prost::Name for EventDefineResolver {
const NAME: &'static str = "EventDefineResolver";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// EventRegisterResolver is an event emitted when data is registered to a
/// resolver on chain.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRegisterResolver {
    /// id is the ID of the resolver that the data was registered to.
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// iri is the IRI of the data that was registered.
    #[prost(string, tag="2")]
    pub iri: ::prost::alloc::string::String,
}
impl ::prost::Name for EventRegisterResolver {
const NAME: &'static str = "EventRegisterResolver";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// ContentHash specifies a hash-based content identifier for a piece of data.
/// Exactly one of its fields must be set so this message behaves like a oneof.
/// A protobuf oneof was not used because this caused compatibility issues with
/// amino signing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentHash {
    /// raw specifies "raw" data which does not specify a deterministic, canonical
    /// encoding. Users of these hashes MUST maintain a copy of the hashed data
    /// which is preserved bit by bit. All other content encodings specify a
    /// deterministic, canonical encoding allowing implementations to choose from a
    /// variety of alternative formats for transport and encoding while maintaining
    /// the guarantee that the canonical hash will not change.
    #[prost(message, optional, tag="1")]
    pub raw: ::core::option::Option<content_hash::Raw>,
    /// graph specifies graph data that conforms to the RDF data model.
    /// The canonicalization algorithm used for an RDF graph is specified by
    /// GraphCanonicalizationAlgorithm.
    #[prost(message, optional, tag="2")]
    pub graph: ::core::option::Option<content_hash::Graph>,
}
/// Nested message and enum types in `ContentHash`.
pub mod content_hash {
    /// RawVis the content hash type used for raw data.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Raw {
        /// hash represents the hash of the data based on the specified
        /// digest_algorithm. It must be at least 20 bytes long and at most 64 bytes long.
        #[prost(bytes="vec", tag="1")]
        pub hash: ::prost::alloc::vec::Vec<u8>,
        /// digest_algorithm represents the hash digest algorithm and should be a non-zero value from the DigestAlgorithm enum.
        #[prost(uint32, tag="2")]
        pub digest_algorithm: u32,
        /// file_extension represents the file extension for raw data. It can be
        /// must be between 2-6 characters long, must be all lower-case and should represent
        /// the canonical extension for the media type.
        ///
        /// A list of canonical extensions which should be used is provided here
        /// and SHOULD be used by implementations: txt, json, csv, xml, pdf, tiff,
        /// jpg, png, svg, webp, avif, gif, apng, mpeg, mp4, webm, ogg, heic, raw.
        ///
        /// The above list should be updated as new media types come into common usage
        /// especially when there are two or more possible extensions (i.e. jpg vs jpeg or tif vs tiff).
        #[prost(string, tag="3")]
        pub file_extension: ::prost::alloc::string::String,
    }
impl ::prost::Name for Raw {
const NAME: &'static str = "Raw";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.ContentHash.{}", Self::NAME)
            }}
    /// Graph is the content hash type used for RDF graph data.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Graph {
        /// hash represents the hash of the data based on the specified
        /// digest_algorithm. It must be at least 20 bytes long and at most 64 bytes long.
        #[prost(bytes="vec", tag="1")]
        pub hash: ::prost::alloc::vec::Vec<u8>,
        /// digest_algorithm represents the hash digest algorithm and should be a non-zero value from the DigestAlgorithm enum.
        #[prost(uint32, tag="2")]
        pub digest_algorithm: u32,
        /// graph_canonicalization_algorithm represents the RDF graph
        /// canonicalization algorithm and should be a non-zero value from the GraphCanonicalizationAlgorithm enum.
        #[prost(uint32, tag="3")]
        pub canonicalization_algorithm: u32,
        /// merkle_tree is the merkle tree type used for the graph hash, if any and should be a value from the GraphMerkleTree enum
        /// or left unspecified.
        #[prost(uint32, tag="4")]
        pub merkle_tree: u32,
    }
impl ::prost::Name for Graph {
const NAME: &'static str = "Graph";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.ContentHash.{}", Self::NAME)
            }}
}
impl ::prost::Name for ContentHash {
const NAME: &'static str = "ContentHash";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// ContentHashes contains list of content ContentHash.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentHashes {
    /// data is a list of content hashes which the resolver claims to serve.
    #[prost(message, repeated, tag="1")]
    pub content_hashes: ::prost::alloc::vec::Vec<ContentHash>,
}
impl ::prost::Name for ContentHashes {
const NAME: &'static str = "ContentHashes";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// DigestAlgorithm is the hash digest algorithm
///
/// With v2, this enum is no longer validated on-chain.
/// However, this enum SHOULD still be used and updated as a registry of known digest
/// algorithms and all implementations should coordinate on these values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DigestAlgorithm {
    /// unspecified and invalid
    Unspecified = 0,
    /// BLAKE2b-256
    Blake2b256 = 1,
}
impl DigestAlgorithm {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DigestAlgorithm::Unspecified => "DIGEST_ALGORITHM_UNSPECIFIED",
            DigestAlgorithm::Blake2b256 => "DIGEST_ALGORITHM_BLAKE2B_256",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DIGEST_ALGORITHM_UNSPECIFIED" => Some(Self::Unspecified),
            "DIGEST_ALGORITHM_BLAKE2B_256" => Some(Self::Blake2b256),
            _ => None,
        }
    }
}
/// GraphCanonicalizationAlgorithm is the graph canonicalization algorithm
///
/// With v2, this enum is no longer validated on-chain.
/// However, this enum SHOULD still be used and updated as a registry of known canonicalization
/// algorithms and all implementations should coordinate on these values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GraphCanonicalizationAlgorithm {
    /// unspecified and invalid
    Unspecified = 0,
    /// RDFC 1.0 graph canonicalization algorithm. Essentially the same as URDNA2015 with some
    /// small clarifications around escaping of escape characters.
    Rdfc10 = 1,
}
impl GraphCanonicalizationAlgorithm {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GraphCanonicalizationAlgorithm::Unspecified => "GRAPH_CANONICALIZATION_ALGORITHM_UNSPECIFIED",
            GraphCanonicalizationAlgorithm::Rdfc10 => "GRAPH_CANONICALIZATION_ALGORITHM_RDFC_1_0",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GRAPH_CANONICALIZATION_ALGORITHM_UNSPECIFIED" => Some(Self::Unspecified),
            "GRAPH_CANONICALIZATION_ALGORITHM_RDFC_1_0" => Some(Self::Rdfc10),
            _ => None,
        }
    }
}
/// GraphMerkleTree is the graph merkle tree type used for hashing, if any.
///
/// With v2, this enum is no longer validated on-chain.
/// However, this enum SHOULD still be used and updated as a registry of known merkle tree
/// types and all implementations should coordinate on these values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GraphMerkleTree {
    /// unspecified and valid
    NoneUnspecified = 0,
}
impl GraphMerkleTree {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GraphMerkleTree::NoneUnspecified => "GRAPH_MERKLE_TREE_NONE_UNSPECIFIED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GRAPH_MERKLE_TREE_NONE_UNSPECIFIED" => Some(Self::NoneUnspecified),
            _ => None,
        }
    }
}
/// QueryAnchorByIRIRequest is the Query/AnchorByIRI request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnchorByIriRequest {
    /// iri is the IRI of the anchored data.
    #[prost(string, tag="1")]
    pub iri: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryAnchorByIriRequest {
const NAME: &'static str = "QueryAnchorByIRIRequest";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryAnchorByIRIResponse is the Query/AnchorByIRI response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnchorByIriResponse {
    /// anchor is information about the data anchor.
    #[prost(message, optional, tag="1")]
    pub anchor: ::core::option::Option<AnchorInfo>,
}
impl ::prost::Name for QueryAnchorByIriResponse {
const NAME: &'static str = "QueryAnchorByIRIResponse";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryAnchorByHashRequest is the Query/AnchorByHash request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnchorByHashRequest {
    /// content_hash is the ContentHash of the anchored data.
    #[prost(message, optional, tag="1")]
    pub content_hash: ::core::option::Option<ContentHash>,
}
impl ::prost::Name for QueryAnchorByHashRequest {
const NAME: &'static str = "QueryAnchorByHashRequest";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryAnchorByHashResponse is the Query/AnchorByHash response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnchorByHashResponse {
    /// anchor is information about the data anchor.
    #[prost(message, optional, tag="1")]
    pub anchor: ::core::option::Option<AnchorInfo>,
}
impl ::prost::Name for QueryAnchorByHashResponse {
const NAME: &'static str = "QueryAnchorByHashResponse";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryAttestationsByAttestorRequest is the Query/AttestationsByAttestor
/// request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAttestationsByAttestorRequest {
    /// attestor is the address of the attestor.
    #[prost(string, tag="1")]
    pub attestor: ::prost::alloc::string::String,
    /// pagination is the PageRequest to use for pagination.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryAttestationsByAttestorRequest {
const NAME: &'static str = "QueryAttestationsByAttestorRequest";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryAttestationsByAttestorResponse is the Query/AttestationsByAttestor
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAttestationsByAttestorResponse {
    /// attestations are the attestations by the attestor.
    #[prost(message, repeated, tag="1")]
    pub attestations: ::prost::alloc::vec::Vec<AttestationInfo>,
    /// pagination is the pagination PageResponse.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryAttestationsByAttestorResponse {
const NAME: &'static str = "QueryAttestationsByAttestorResponse";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryAttestationsByIRIRequest is the Query/AttestationsByIRI request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAttestationsByIriRequest {
    /// iri is the IRI of the anchored data.
    #[prost(string, tag="1")]
    pub iri: ::prost::alloc::string::String,
    /// pagination is the PageRequest to use for pagination.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryAttestationsByIriRequest {
const NAME: &'static str = "QueryAttestationsByIRIRequest";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryAttestationsByIRIResponse is the Query/AttestationsByIRI response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAttestationsByIriResponse {
    /// attestations are the attestations that have been made to the anchored data.
    #[prost(message, repeated, tag="1")]
    pub attestations: ::prost::alloc::vec::Vec<AttestationInfo>,
    /// pagination is the pagination PageResponse.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryAttestationsByIriResponse {
const NAME: &'static str = "QueryAttestationsByIRIResponse";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryAttestationsByHashRequest is the Query/AttestationsByHash request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAttestationsByHashRequest {
    /// content_hash is the ContentHash of the anchored data.
    #[prost(message, optional, tag="1")]
    pub content_hash: ::core::option::Option<ContentHash>,
    /// pagination is the PageRequest to use for pagination.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryAttestationsByHashRequest {
const NAME: &'static str = "QueryAttestationsByHashRequest";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryAttestationsByHashResponse is the Query/AttestationsByHash response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAttestationsByHashResponse {
    /// attestations are the attestations that have been made to the anchored data.
    #[prost(message, repeated, tag="1")]
    pub attestations: ::prost::alloc::vec::Vec<AttestationInfo>,
    /// pagination is the pagination PageResponse.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryAttestationsByHashResponse {
const NAME: &'static str = "QueryAttestationsByHashResponse";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryResolverRequest is the Query/Resolver request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResolverRequest {
    /// id is the ID of the resolver.
    #[prost(uint64, tag="1")]
    pub id: u64,
}
impl ::prost::Name for QueryResolverRequest {
const NAME: &'static str = "QueryResolverRequest";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryResolverResponse is the Query/Resolver response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResolverResponse {
    /// resolver is information about the resolver.
    #[prost(message, optional, tag="1")]
    pub resolver: ::core::option::Option<ResolverInfo>,
}
impl ::prost::Name for QueryResolverResponse {
const NAME: &'static str = "QueryResolverResponse";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryResolversByIRIRequest is the Query/ResolversByIRI request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResolversByIriRequest {
    /// iri is the IRI of the anchored data.
    #[prost(string, tag="1")]
    pub iri: ::prost::alloc::string::String,
    /// pagination is the PageRequest to use for pagination.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryResolversByIriRequest {
const NAME: &'static str = "QueryResolversByIRIRequest";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryResolversByIRIResponse is the Query/ResolversByIRI response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResolversByIriResponse {
    /// resolvers are the resolvers that have registered the anchored data.
    #[prost(message, repeated, tag="1")]
    pub resolvers: ::prost::alloc::vec::Vec<ResolverInfo>,
    /// pagination is the PageResponse to use for pagination.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryResolversByIriResponse {
const NAME: &'static str = "QueryResolversByIRIResponse";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryResolversByHashRequest is the Query/ResolversByHash request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResolversByHashRequest {
    /// content_hash is the ContentHash of the anchored data.
    #[prost(message, optional, tag="1")]
    pub content_hash: ::core::option::Option<ContentHash>,
    /// pagination is the PageRequest to use for pagination.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryResolversByHashRequest {
const NAME: &'static str = "QueryResolversByHashRequest";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryResolversByHashResponse is the Query/ResolversByHash response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResolversByHashResponse {
    /// resolvers are the resolvers that have registered the data.
    #[prost(message, repeated, tag="1")]
    pub resolvers: ::prost::alloc::vec::Vec<ResolverInfo>,
    /// pagination is the PageResponse to use for pagination.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryResolversByHashResponse {
const NAME: &'static str = "QueryResolversByHashResponse";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryResolversByURLRequest is the Query/ResolversByURL request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResolversByUrlRequest {
    /// url is the URL of the resolver.
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
    /// pagination is the PageRequest to use for pagination.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryResolversByUrlRequest {
const NAME: &'static str = "QueryResolversByURLRequest";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// QueryResolversByURLResponse is the Query/ResolversByURL response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResolversByUrlResponse {
    /// resolvers are the resolvers that have a matching URL.
    #[prost(message, repeated, tag="1")]
    pub resolvers: ::prost::alloc::vec::Vec<ResolverInfo>,
    /// pagination is the PageResponse to use for pagination.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<::cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryResolversByUrlResponse {
const NAME: &'static str = "QueryResolversByURLResponse";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// ConvertIRIToHashRequest is the Query/ConvertIRIToHash request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConvertIriToHashRequest {
    /// iri is the IRI to convert to a ContentHash.
    #[prost(string, tag="1")]
    pub iri: ::prost::alloc::string::String,
}
impl ::prost::Name for ConvertIriToHashRequest {
const NAME: &'static str = "ConvertIRIToHashRequest";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// ConvertIRIToHashResponse is the Query/ConvertIRIToHash response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConvertIriToHashResponse {
    /// content_hash is the ContentHash converted from the IRI.
    #[prost(message, optional, tag="1")]
    pub content_hash: ::core::option::Option<ContentHash>,
}
impl ::prost::Name for ConvertIriToHashResponse {
const NAME: &'static str = "ConvertIRIToHashResponse";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// ConvertHashToIRIRequest is the Query/ConvertHashToIRI request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConvertHashToIriRequest {
    /// content_hash is the ContentHash to convert to an IRI.
    #[prost(message, optional, tag="1")]
    pub content_hash: ::core::option::Option<ContentHash>,
}
impl ::prost::Name for ConvertHashToIriRequest {
const NAME: &'static str = "ConvertHashToIRIRequest";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// ConvertHashToIRIResponse is the Query/ConvertHashToIRI response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConvertHashToIriResponse {
    /// iri is the IRI converted from the ContentHash.
    #[prost(string, tag="1")]
    pub iri: ::prost::alloc::string::String,
}
impl ::prost::Name for ConvertHashToIriResponse {
const NAME: &'static str = "ConvertHashToIRIResponse";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// AnchorInfo is the information for a data anchor.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnchorInfo {
    /// iri is the IRI of the anchored data.
    #[prost(string, tag="1")]
    pub iri: ::prost::alloc::string::String,
    /// content_hash is the ContentHash of the anchored data.
    #[prost(message, optional, tag="2")]
    pub content_hash: ::core::option::Option<ContentHash>,
    /// timestamp is the time at which the data was anchored.
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for AnchorInfo {
const NAME: &'static str = "AnchorInfo";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// AttestationInfo is the information for an attestation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttestationInfo {
    /// iri is the IRI of the anchored data.
    #[prost(string, tag="1")]
    pub iri: ::prost::alloc::string::String,
    /// attestor is the address of the account that attested to the anchored data.
    #[prost(string, tag="2")]
    pub attestor: ::prost::alloc::string::String,
    /// timestamp is the time at which the data was attested to.
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for AttestationInfo {
const NAME: &'static str = "AttestationInfo";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// ResolverInfo is the information for a resolver.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolverInfo {
    /// id is the ID of the resolver.
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// url is the URL of the resolver.
    #[prost(string, tag="2")]
    pub url: ::prost::alloc::string::String,
    /// manager is the address of the account that manages the resolver.
    /// This will be empty if the resolver is public.
    #[prost(string, tag="3")]
    pub manager: ::prost::alloc::string::String,
}
impl ::prost::Name for ResolverInfo {
const NAME: &'static str = "ResolverInfo";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// DataID stores a compact data ID and its full IRI.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataId {
    /// id is the compact automatically-generated data ID.
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// iri is the IRI of the data which contains its full ContentHash.
    #[prost(string, tag="2")]
    pub iri: ::prost::alloc::string::String,
}
impl ::prost::Name for DataId {
const NAME: &'static str = "DataID";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// DataAnchor stores the anchor timestamp for a data object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataAnchor {
    /// id is the compact data ID.
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// timestamp is the anchor timestamp for this object - the time at which
    /// it was first known to the blockchain.
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for DataAnchor {
const NAME: &'static str = "DataAnchor";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// DataAttestor is a join table for associating data IDs and attestors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataAttestor {
    /// id is the compact data ID.
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// attestor is the account address of the attestor.
    #[prost(bytes="vec", tag="2")]
    pub attestor: ::prost::alloc::vec::Vec<u8>,
    /// timestamp is the time at which the attestor signed this data object.
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for DataAttestor {
const NAME: &'static str = "DataAttestor";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// Resolver describes a data resolver.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resolver {
    /// id is the ID of the resolver.
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// url is the URL of the resolver.
    #[prost(string, tag="2")]
    pub url: ::prost::alloc::string::String,
    /// manager is the bytes address of the resolver manager who defined
    /// this resolver. If the resolver is public, then this field is empty.
    #[prost(bytes="vec", tag="3")]
    pub manager: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for Resolver {
const NAME: &'static str = "Resolver";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// DataResolver is a join table between data objects and resolvers and indicates
/// that a resolver claims to be able to resolve this data object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataResolver {
    /// id is the compact data ID.
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// resolver_id is the ID of the resolver.
    #[prost(uint64, tag="2")]
    pub resolver_id: u64,
}
impl ::prost::Name for DataResolver {
const NAME: &'static str = "DataResolver";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// MsgAnchor is the Msg/Anchor request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAnchor {
    /// sender is the address of the sender of the transaction. The sender in
    /// Anchor is not attesting to the veracity of the underlying data. They
    /// can simply be an intermediary providing services.
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// content_hash is the content hash for the data to anchor.
    #[prost(message, optional, tag="2")]
    pub content_hash: ::core::option::Option<ContentHash>,
}
impl ::prost::Name for MsgAnchor {
const NAME: &'static str = "MsgAnchor";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// MsgAnchor is the Msg/Anchor response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAnchorResponse {
    /// iri is the IRI of the data that was anchored.
    #[prost(string, tag="1")]
    pub iri: ::prost::alloc::string::String,
    /// timestamp is the time at which the data was anchored.
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for MsgAnchorResponse {
const NAME: &'static str = "MsgAnchorResponse";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// MsgAttest is the Msg/Attest request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAttest {
    /// attestor is the addresses of the account attesting to the veracity of the
    /// data. By making an Attest request, the attestor is attesting to the
    /// veracity of the data referenced by the IRI. The precise meaning of this may
    /// vary depending on the underlying data.
    #[prost(string, tag="1")]
    pub attestor: ::prost::alloc::string::String,
    /// content_hashes are the content hashes for anchored data. Only RDF graph
    /// data can be signed as its data model is intended to specifically convey
    /// semantic meaning.
    #[prost(message, repeated, tag="2")]
    pub content_hashes: ::prost::alloc::vec::Vec<content_hash::Graph>,
}
impl ::prost::Name for MsgAttest {
const NAME: &'static str = "MsgAttest";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// MsgAttestResponse is the Msg/Attest response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAttestResponse {
    /// iris are the IRIs of the data that was attested to. If the attestor attests
    /// to the same piece of data, the previous attestation will not be updated and
    /// the IRI will not be included in this list.
    #[prost(string, repeated, tag="1")]
    pub iris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// timestamp is the time at which any new attestations were made.
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for MsgAttestResponse {
const NAME: &'static str = "MsgAttestResponse";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// MsgDefineResolver is the Msg/DefineResolver request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDefineResolver {
    /// definer is the address of the account defining the resolver. If
    /// the boolean public is set to true, then any user can register
    /// data with this resolver. If the boolean public is set to false,
    /// then only the definer can register data with this resolver and
    /// must use a feature such as cosmos.authz to authorize other accounts
    /// to register data with this resolver.
    #[prost(string, tag="1")]
    pub definer: ::prost::alloc::string::String,
    /// resolver_url is a resolver URL.
    ///
    /// If it refers to an HTTP URL, that HTTP service should
    /// respond to a GET request with the IRI of a ContentHash as the path parameter
    /// and return the content if it exists or a 404. For graph data, resolvers
    /// should use the HTTP Accept header to negotiate the RDF serialization
    /// format.
    ///
    /// To use IPFS, the resolver_url ipfs: should be defined with public set to true
    /// and used as the resolver for any data hosted on IPFS. Content hashes must be
    /// adapted to IPFS's CID format. The multicodec raw (0x55) should
    /// be used for all raw content hashes and the multicodec rdfc-1 (0xb403)
    /// should be used for all graph content hashes (unless new canonicalization
    /// or merkle tree algorithms are used which may or may not be supported
    /// by IPFS). Note that IPFS's tools currently do not support creating or
    /// resolving RDFC-1 content hashes so upstream work will be needed for
    /// that integration to be fully supported.
    #[prost(string, tag="2")]
    pub resolver_url: ::prost::alloc::string::String,
    /// public is a boolean indicating whether the resolver is public or not.
    /// If public is false then only the definer can register data with this
    /// resolver.
    #[prost(bool, tag="3")]
    pub public: bool,
}
impl ::prost::Name for MsgDefineResolver {
const NAME: &'static str = "MsgDefineResolver";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// MsgDefineResolverResponse is the Msg/DefineResolver response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDefineResolverResponse {
    /// resolver_id is the integer ID of the resolver to be used in
    /// MsgRegisterResolver.
    #[prost(uint64, tag="1")]
    pub resolver_id: u64,
}
impl ::prost::Name for MsgDefineResolverResponse {
const NAME: &'static str = "MsgDefineResolverResponse";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// MsgRegisterResolver is the Msg/RegisterResolver request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterResolver {
    /// signer is the address registering data with the resolver. If
    /// the resolver is not public then the signer must be the definer
    /// of the resolver.
    #[prost(string, tag="1")]
    pub signer: ::prost::alloc::string::String,
    /// resolver_id is the ID of a resolver defined with Msg/DefineResolver.
    #[prost(uint64, tag="2")]
    pub resolver_id: u64,
    /// content_hashes is a list of content hashes which the resolver claims to
    /// serve.
    #[prost(message, repeated, tag="3")]
    pub content_hashes: ::prost::alloc::vec::Vec<ContentHash>,
}
impl ::prost::Name for MsgRegisterResolver {
const NAME: &'static str = "MsgRegisterResolver";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
/// MsgRegisterResolverResponse is the Msg/RegisterResolver response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterResolverResponse {
}
impl ::prost::Name for MsgRegisterResolverResponse {
const NAME: &'static str = "MsgRegisterResolverResponse";
const PACKAGE: &'static str = "regen.data.v2";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("regen.data.v2.{}", Self::NAME)
            }}
include!("regen.data.v2.serde.rs");
include!("regen.data.v2.tonic.rs");
// @@protoc_insertion_point(module)