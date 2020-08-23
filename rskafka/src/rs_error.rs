use std::fmt::Display;

#[derive(Debug)]
pub enum RsLocalError {
    // Bad message
    BadMsg = -198,

    /// Fail
    Fail = -196,

    /// Transport
    Transport = -195,

    TooLarge = -50,

    Codec = -49,
}

impl Display for RsLocalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum RsBrokerError {
    /// Success
    NoError = 0,

    /// Offset out of range
    OffsetOutOfRange = 1,

    /// Invalid message
    InvalidMsg = 2,

    /// Unknown topic or partition
    UnknownTopicOrPart = 3,

    /// Invalid message size
    InvalidMsgSize = 4,

    /// Leader not available
    LeaderNotAvailable = 5,

    /// Not leader for partition
    NotLeaderForPartition = 6,

    /// Request timed out
    RequestTimedOut = 7,

    /// Broker not available
    BrokerNotAvailable = 8,

    /// Replica not available
    ReplicaNotAvailable = 9,

    /// Message size too large
    MsgSizeTooLarge = 10,

    /// StaleControllerEpochCode
    StaleCtrlEpoch = 11,

    /// Offset metadata string too large
    OffsetMetadataTooLarge = 12,

    /// Broker disconnected before response received
    NetworkException = 13,

    /// Group coordinator load in progress
    GroupLoadInProress = 14,

    /// Group coordinator not available
    GroupCoordinatorNotAvailable = 15,

    /// Not coordinator for group
    NotCoordinatorForGroup = 16,

    /// Invalid topic
    TopicException = 17,

    /// Message batch larger than configured server segment size
    RecordListTooLarge = 18,

    /// Not enough in-sync replicas
    NotEnoughReplicas = 19,

    /// Message(s) written to insufficient number of in-sync replicas
    NotEnoughReplicasAfterAppend = 20,

    /// Invalid required acks value
    InvalidRequiredAcks = 21,

    /// Specified group generation id is not valid
    IllegalGeneration = 22,

    /// Inconsistent group protocol
    InconsistentGroupProtocol = 23,

    /// Invalid group.id
    InvalidGroupId = 24,

    /// Unknown member
    UnknownMemberId = 25,

    /// Invalid session timeout
    InvalidSessionTimeout = 26,

    /// Group rebalance in progress
    RebalanceInProgress = 27,

    /// Commit offset data size is not valid
    InvalidCommitOffsetSize = 28,

    /// Topic authorization failed
    TopicAuthorizationFailed = 29,

    /// Group authorization failed
    GroupAuthorizationFailed = 30,

    /// Cluster authorization failed
    ClusterAuthorizationFailed = 31,

    /// Invalid timestamp
    InvalidTimestamp = 32,

    /// Unsupported SASL mechanism
    UnsupportedSaslMechanism = 33,

    /// Illegal SASL state
    IllegalSaslState = 34,

    /// Unsupported version
    UnsupportedVersion = 35,

    /// Topic already exists
    TopicAlreadyExists = 36,

    /// Invalid number of partitions
    InvalidPartitions = 37,

    ///Invalid replication factor
    InvalidReplicationFactor = 38,

    /// Invalid replica assignment
    InvalidReplicaAssignment = 39,

    /// Invalid config
    InvalidConfig = 40,

    /// Not controller for cluster
    NotController = 41,

    /// Invalid request
    InvalidRequest = 42,

    /// Message format on broker does not support request
    UnsupportedForMessageFormat = 43,

    /// Isolation policy violation
    PolicyViolation = 44,

    /// Broker received an out of order sequence number
    OutOfOrderSequenceNumber = 45,

    /// Broker received a duplicate sequence number
    DuplicateSequenceNumber = 46,

    /// Producer attempted an operation with an old epoch
    InvalidProducerEpoch = 47,

    /// Producer attempted a transactional operation in an invalid state
    InvalidTxnState = 48,

    /// Producer attempted to use a producer id which is not currently assigned to its transactional id
    InvalidProducerIdMapping = 49,

    /// Transaction timeout is larger than the maximum value allowed by the broker's max.transaction.timeout.ms
    InvalidTransactionTimeout = 50,

    /// Producer attempted to update a transaction while another concurrent operation on the same transaction was ongoing
    ConcurrentTransactions = 51,

    /// Indicates that the transaction coordinator sending a WriteTxnMarker is no longer the current coordinator for a given producer
    TransactionCoordinatorFenced = 52,

    /// Transactional Id authorization failed
    TransactionalIdAuthorizationFailed = 53,

    /// Security features are disabled
    SecurityDisabled = 54,

    /// Operation not attempted
    OperationNotAttempted = 55,

    /// Disk error when trying to access log file on the disk.
    KafkaStorageError = 56,

    /// The user-specified log directory is not found in the broker config.
    LogDirNotFound = 57,

    /// SASL Authentication failed.
    SaslAuthenticationFailed = 58,

    /// Unknown Producer Id.
    UnknownProducerId = 59,

    /// Partition reassignment is in progress.
    ReassignmentInProgress = 60,

    /// Delegation Token feature is not enabled.
    DelegationTokenAuthDisabled = 61,

    /// Delegation Token is not found on server.
    DelegationTokenNotFound = 62,

    /// Specified Principal is not valid Owner/Renewer.
    DelegationTokenOwnerMismatch = 63, 

    /// Delegation Token requests are not allowed on this connection.
    DelegationTokenRequestNotAllowed = 64,

    /// Delegation Token authorization failed.
    DelegationTokenAuthorizationFailed = 65,

    /// Delegation Token is expired.
    DelegationTokenExpired = 66,

    /// Supplied principalType is not supported.
    InvalidPrincipalType = 67,

    /// The group is not empty.
    NonEmptyGroup = 68,

    /// The group id does not exist.
    GroupIdNotFound = 69,

    /// The fetch session ID was not found.
    FetchSessionIdNotFound = 70,

    /// The fetch session epoch is invalid.
    InvalidFetchSessionEpoch = 71,

    /// No matching listener.
    ListenerNotFound = 72,

    /// Topic deletion is disabled.
    TopicDeletionDisabled = 73,

    /// Unsupported compression type.
    UnsupportedCompressionType = 74
}

#[derive(Debug)]
pub enum ErrorCode {
    Local(RsLocalError),
    Broker(RsBrokerError)
}


/// rskafka user facing error.
#[derive(Debug)]
pub struct RsError
{
    /// Error code
    code: ErrorCode,

    /// Human readable error string.
    reason: String,

    /// This is a fatal error (client is now in an unusable state).
    fatal: bool,

    /// Operation is retriable.
    retriable: bool,

    /// An abortable transaction error.
    txn_requires_abort: bool,
}


impl RsError {
    pub fn code(&self) -> &ErrorCode { &self.code }
    pub fn reason(&self) -> &String { &self.reason }
    pub fn to_string(&self) -> String { format!("[{:?}] {}", self.code, self.reason).into() }
}

pub fn from_local(code: RsLocalError) -> RsError { RsError { code: ErrorCode::Local(code), reason: "".into(), fatal: false, retriable: false, txn_requires_abort: false } }