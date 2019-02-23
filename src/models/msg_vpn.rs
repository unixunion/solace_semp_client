/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see [note 1](#notes)) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+  broker. Resources are either individual **objects**, or **collections** of  objects. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Configuration|/SEMP/v2/config|Reading and writing config state|See [note 2](#notes)    Resources are always nouns, with individual objects being singular and  collections being plural. Objects within a collection are identified by an  `obj-id`, which follows the collection name with the form  `collection-name/obj-id`. Some examples:  <pre> /SEMP/v2/config/msgVpns                       ; MsgVpn collection /SEMP/v2/config/msgVpns/finance               ; MsgVpn object named \"finance\" /SEMP/v2/config/msgVpns/finance/queues        ; Queue collection within MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance/queues/orderQ ; Queue object named \"orderQ\" within MsgVpn \"finance\" </pre>  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and  are described by JSON arrays. Each item in the array represents an object in  the same manner as the individual object would normally be represented. The creation of a new object is done through its collection  resource.   ## Object Resources  Objects are composed of attributes and collections, and are described by JSON  content as name/value pairs. The collections of an object are not contained  directly in the object's JSON content, rather the content includes a URI  attribute which points to the collection. This contained collection resource  must be managed as a separate resource through this URI.  At a minimum, every object has 1 or more identifying attributes, and its own  `uri` attribute which contains the URI to itself. Attributes may have any  (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See [note 3](#notes) Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in  certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request     ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these  general principles:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see [note 4](#notes)) PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many  method/URI combinations. Individual URIs may document additional parameters.  Note that multiple query parameters can be used together in a single URI,  separated by the ampersand character. For example:  <pre> ; Request for the MsgVpns collection using two hypothetical query parameters ; \"q1\" and \"q2\" with values \"val1\" and \"val2\" respectively /SEMP/v2/config/msgVpns?q1=val1&q2=val2 </pre>  ### select  Include in the response only selected attributes of the object, or exclude  from the response selected attributes of the object. Use this query parameter  to limit the size of the returned data for each returned object, return only  those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the  list contains attribute names that are not prefaced by `-`, only those  attributes are included in the response. If the list contains attribute names  that are prefaced by `-`, those attributes are excluded from the response. If  the list contains both types, then the difference of the first set of  attributes and the second set of attributes is returned. If the list is  empty (i.e. `select=`), no attributes are returned  All attributes that are prefaced by `-` must follow all attributes that are  not prefaced by `-`. In addition, each attribute name in the list must match  at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute  names are supported using periods (e.g. `parentName.childName`).  Some examples:  <pre> ; List of all MsgVpn names /SEMP/v2/config/msgVpns?select=msgVpnName  ; List of all MsgVpn and their attributes except for their names /SEMP/v2/config/msgVpns?select=-msgVpnName  ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance?select=authentication*  ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/config/msgVpns/finance?select=-authentication*  ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance/queues/orderQ?select=owner,permission </pre>  ### where  Include in the response only objects where certain conditions are true. Use  this query parameter to limit which objects are returned to those whose  attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions  must be true for the object to be included in the response. Each expression  takes the form:  <pre> expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' </pre>  `value` may be a number, string, `true`, or `false`, as appropriate for the  type of `attribute-name`. Greater-than and less-than comparisons only work for  numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more  characters). Some examples:  <pre> ; Only enabled MsgVpns /SEMP/v2/config/msgVpns?where=enabled==true  ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/config/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap  ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/config/msgVpns?where=maxConnectionCount>100  ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/config/msgVpns?where=msgVpnName==B* </pre>  ### count  Limit the count of objects in the response. This can be useful to limit the  size of the response for large collections. The minimum value for `count` is  `1` and the default is `10`. There is a hidden maximum  as to prevent overloading the system. For example:  <pre> ; Up to 25 MsgVpns /SEMP/v2/config/msgVpns?count=25 </pre>  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data  that should not be created or interpreted by SEMP clients, and should only be  used as described below.  When a request is made for a collection and there may be additional objects  available for retrieval that are not included in the initial response, the  response will include a `cursorQuery` field containing a cursor. The value  of this field can be specified in the `cursor` query parameter of a  subsequent request to retrieve the next page of objects. For convenience,  an appropriate URI is constructed automatically by the broker and included  in the `nextPageUri` field of the response. This URI can be used directly  to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first. 5|For DELETE, the body of the request currently serves no purpose and will cause an error if not empty.    
 *
 * OpenAPI spec version: 2.10
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgVpn {
  /// Enable or disable Basic Authentication for clients connecting to the Message VPN. The default value is `true`.
  #[serde(rename = "authenticationBasicEnabled")]
  authentication_basic_enabled: Option<bool>,
  /// The name of the RADIUS or LDAP Profile to use when \"authenticationBasicType\" is \"radius\" or \"ldap\" respectively. The default value is `\"default\"`.
  #[serde(rename = "authenticationBasicProfileName")]
  authentication_basic_profile_name: Option<String>,
  /// The RADIUS domain string to use when \"authenticationBasicType\" is \"radius\". The default value is `\"\"`.
  #[serde(rename = "authenticationBasicRadiusDomain")]
  authentication_basic_radius_domain: Option<String>,
  /// Authentication mechanism to be used for Basic Authentication of clients connecting to the Message VPN. The default value is `\"radius\"`. The allowed values and their meaning are:  <pre> \"internal\" - Internal database. Authentication is against Client Usernames. \"ldap\" - LDAP authentication. An LDAP profile name must be provided. \"radius\" - RADIUS authentication. A RADIUS profile name must be provided. \"none\" - No authentication. Anonymous login allowed. </pre> 
  #[serde(rename = "authenticationBasicType")]
  authentication_basic_type: Option<String>,
  /// When enabled, if the client specifies a Client Username via the API connect method, the client provided Username is used instead of the CN (Common Name) field of the certificate\"s subject. When disabled, the certificate CN is always used as the Client Username. The default value is `false`.
  #[serde(rename = "authenticationClientCertAllowApiProvidedUsernameEnabled")]
  authentication_client_cert_allow_api_provided_username_enabled: Option<bool>,
  /// Enable or disable the Client Certificate client Authentication for the Message VPN. The default value is `false`.
  #[serde(rename = "authenticationClientCertEnabled")]
  authentication_client_cert_enabled: Option<bool>,
  /// The maximum depth for the client certificate chain. The depth of the chain is defined as the number of signing CA certificates that are present in the chain back to the trusted self-signed root CA certificate. The default value is `3`.
  #[serde(rename = "authenticationClientCertMaxChainDepth")]
  authentication_client_cert_max_chain_depth: Option<i64>,
  /// Define overrides for certificate revocation checking. For \"allow-all\" setting, the result of the client certificate revocation check is ignored. For \"allow-unknown\" setting, the client is authenticated even if the revocation status of his certificate cannot be determined. For \"allow-valid\" setting, the client is only authenticated if the revocation check returned an explicit positive response. The default value is `\"allow-valid\"`. The allowed values and their meaning are:  <pre> \"allow-all\" - Allow the client to authenticate, the result of client certificate revocation check is ingored. \"allow-unknown\" - Allow the client to authenticate even if the revocation status of his certificate cannot be determined. \"allow-valid\" - Allow the client to authenticate only when the revocation check returned an explicit positive response. </pre>  Available since 2.6.
  #[serde(rename = "authenticationClientCertRevocationCheckMode")]
  authentication_client_cert_revocation_check_mode: Option<String>,
  /// The field from the client certificate to use as the client username. The default value is `\"common-name\"`. The allowed values and their meaning are:  <pre> \"common-name\" - the username is extracted from the certificate's Common Name. \"subject-alternate-name-msupn\" - the username is extracted from the certificate's Other Name type of the Subject Alternative Name and must have the msUPN signature. </pre>  Available since 2.5.
  #[serde(rename = "authenticationClientCertUsernameSource")]
  authentication_client_cert_username_source: Option<String>,
  /// Enable or disable validation of the \"Not Before\" and \"Not After\" validity dates in the client certificate. When disabled, a certificate will be accepted even if the certificate is not valid according to the \"Not Before\" and \"Not After\" validity dates in the certificate. The default value is `true`.
  #[serde(rename = "authenticationClientCertValidateDateEnabled")]
  authentication_client_cert_validate_date_enabled: Option<bool>,
  /// When enabled, if the client specifies a Client Username via the API connect method, the client provided Username is used instead of the Kerberos Principal name in Kerberos token. When disabled, the Kerberos Principal name is always used as the Client Username. The default value is `false`.
  #[serde(rename = "authenticationKerberosAllowApiProvidedUsernameEnabled")]
  authentication_kerberos_allow_api_provided_username_enabled: Option<bool>,
  /// Enable or disable Kerberos Authentication for clients in the Message VPN. If a user provides credentials for a different authentication scheme, this setting is not applicable. The default value is `false`.
  #[serde(rename = "authenticationKerberosEnabled")]
  authentication_kerberos_enabled: Option<bool>,
  /// The name of the attribute that should be retrieved from the LDAP server as part of the LDAP search when authorizing a client. It indicates that the client belongs to a particular group (i.e. the value associated with this attribute). The default value is `\"memberOf\"`.
  #[serde(rename = "authorizationLdapGroupMembershipAttributeName")]
  authorization_ldap_group_membership_attribute_name: Option<String>,
  /// The LDAP Profile name to be used when \"authorizationType\" is \"ldap\". The default value is `\"\"`.
  #[serde(rename = "authorizationProfileName")]
  authorization_profile_name: Option<String>,
  /// Authorization mechanism to be used for clients connecting to the Message VPN. The default value is `\"internal\"`. The allowed values and their meaning are:  <pre> \"ldap\" - LDAP authorization. \"internal\" - Internal authorization. </pre> 
  #[serde(rename = "authorizationType")]
  authorization_type: Option<String>,
  /// Enable or disable validation of the Common Name (CN) in the server certificate from the Remote Router. If enabled, the Common Name is checked against the list of Trusted Common Names configured for the Bridge. The default value is `true`.
  #[serde(rename = "bridgingTlsServerCertEnforceTrustedCommonNameEnabled")]
  bridging_tls_server_cert_enforce_trusted_common_name_enabled: Option<bool>,
  /// The maximum depth for the server certificate chain. The depth of the chain is defined as the number of signing CA certificates that are present in the chain back to the trusted self-signed root CA certificate. The default value is `3`.
  #[serde(rename = "bridgingTlsServerCertMaxChainDepth")]
  bridging_tls_server_cert_max_chain_depth: Option<i64>,
  /// Enable or disable validation of the \"Not Before\" and \"Not After\" validity dates in the server certificate. When disabled, a certificate will be accepted even if the certificate is not valid according to the \"Not Before\" and \"Not After\" validity dates in the certificate. The default value is `true`.
  #[serde(rename = "bridgingTlsServerCertValidateDateEnabled")]
  bridging_tls_server_cert_validate_date_enabled: Option<bool>,
  /// Enable or disable managing of cache instances over the message bus. The default value is `true`.
  #[serde(rename = "distributedCacheManagementEnabled")]
  distributed_cache_management_enabled: Option<bool>,
  /// Enable or disable the Message VPN. The default value is `false`.
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  #[serde(rename = "eventConnectionCountThreshold")]
  event_connection_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventEgressFlowCountThreshold")]
  event_egress_flow_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventEgressMsgRateThreshold")]
  event_egress_msg_rate_threshold: Option<::models::EventThresholdByValue>,
  #[serde(rename = "eventEndpointCountThreshold")]
  event_endpoint_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventIngressFlowCountThreshold")]
  event_ingress_flow_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventIngressMsgRateThreshold")]
  event_ingress_msg_rate_threshold: Option<::models::EventThresholdByValue>,
  /// Size in KB for what is being considered a large message for the Message VPN. The default value is `1024`.
  #[serde(rename = "eventLargeMsgThreshold")]
  event_large_msg_threshold: Option<i64>,
  /// A prefix applied to all published Events in the Message VPN. The default value is `\"\"`.
  #[serde(rename = "eventLogTag")]
  event_log_tag: Option<String>,
  #[serde(rename = "eventMsgSpoolUsageThreshold")]
  event_msg_spool_usage_threshold: Option<::models::EventThreshold>,
  /// Enable or disable Client level Event message publishing. The default value is `false`.
  #[serde(rename = "eventPublishClientEnabled")]
  event_publish_client_enabled: Option<bool>,
  /// Enable or disable Message VPN level Event message publishing. The default value is `false`.
  #[serde(rename = "eventPublishMsgVpnEnabled")]
  event_publish_msg_vpn_enabled: Option<bool>,
  /// Subscription level Event message publishing mode. The default value is `\"off\"`. The allowed values and their meaning are:  <pre> \"off\" - Disable client level event message publishing. \"on-with-format-v1\" - Enable client level event message publishing with format v1. \"on-with-no-unsubscribe-events-on-disconnect-format-v1\" - As \"on-with-format-v1\", but unsubscribe events are not generated when a client disconnects. Unsubscribe events are still raised when a client explicitly unsubscribes from its subscriptions. \"on-with-format-v2\" - Enable client level event message publishing with format v2. \"on-with-no-unsubscribe-events-on-disconnect-format-v2\" - As \"on-with-format-v2\", but unsubscribe events are not generated when a client disconnects. Unsubscribe events are still raised when a client explicitly unsubscribes from its subscriptions. </pre> 
  #[serde(rename = "eventPublishSubscriptionMode")]
  event_publish_subscription_mode: Option<String>,
  /// Enable or disable Event publish topics in MQTT format. The default value is `false`.
  #[serde(rename = "eventPublishTopicFormatMqttEnabled")]
  event_publish_topic_format_mqtt_enabled: Option<bool>,
  /// Enable or disable Event publish topics in SMF format. The default value is `true`.
  #[serde(rename = "eventPublishTopicFormatSmfEnabled")]
  event_publish_topic_format_smf_enabled: Option<bool>,
  #[serde(rename = "eventServiceAmqpConnectionCountThreshold")]
  event_service_amqp_connection_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventServiceMqttConnectionCountThreshold")]
  event_service_mqtt_connection_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventServiceRestIncomingConnectionCountThreshold")]
  event_service_rest_incoming_connection_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventServiceSmfConnectionCountThreshold")]
  event_service_smf_connection_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventServiceWebConnectionCountThreshold")]
  event_service_web_connection_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventSubscriptionCountThreshold")]
  event_subscription_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventTransactedSessionCountThreshold")]
  event_transacted_session_count_threshold: Option<::models::EventThreshold>,
  #[serde(rename = "eventTransactionCountThreshold")]
  event_transaction_count_threshold: Option<::models::EventThreshold>,
  /// Enable or disable the export of subscriptions in the Message VPN to other routers in the network over Neighbor links. The default value is `false`.
  #[serde(rename = "exportSubscriptionsEnabled")]
  export_subscriptions_enabled: Option<bool>,
  /// Enable or disable JNDI access for clients in the Message VPN. The default value is `false`. Available since 2.2.
  #[serde(rename = "jndiEnabled")]
  jndi_enabled: Option<bool>,
  /// The maximum number of client connections that can be simultaneously connected to the Message VPN. This value may be higher than supported by the hardware. The default is the maximum value supported by the hardware. The default is the max value supported by the hardware.
  #[serde(rename = "maxConnectionCount")]
  max_connection_count: Option<i64>,
  /// The maximum number of egress flows that can be created in the Message VPN. The default value is `16000`.
  #[serde(rename = "maxEgressFlowCount")]
  max_egress_flow_count: Option<i64>,
  /// The maximum number of Queues and Topic Endpoints that can be created in the Message VPN. The default value is `16000`.
  #[serde(rename = "maxEndpointCount")]
  max_endpoint_count: Option<i64>,
  /// The maximum number of ingress flows that can be created in the Message VPN. The default value is `16000`.
  #[serde(rename = "maxIngressFlowCount")]
  max_ingress_flow_count: Option<i64>,
  /// The maximum Message Spool usage by the Message VPN, in megabytes. The default value is `0`.
  #[serde(rename = "maxMsgSpoolUsage")]
  max_msg_spool_usage: Option<i64>,
  /// The maximum number of local client subscriptions (both primary and backup) that can be added to the Message VPN. The default varies by platform. The default varies by platform.
  #[serde(rename = "maxSubscriptionCount")]
  max_subscription_count: Option<i64>,
  /// The maximum number of transacted sessions for the Message VPN. The default varies by platform. The default varies by platform.
  #[serde(rename = "maxTransactedSessionCount")]
  max_transacted_session_count: Option<i64>,
  /// The maximum number of transactions for the Message VPN. The default varies by platform. The default varies by platform.
  #[serde(rename = "maxTransactionCount")]
  max_transaction_count: Option<i64>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName")]
  msg_vpn_name: Option<String>,
  /// The acknowledgement (ACK) propagation interval for the Replication Bridge, in number of replicated messages. The default value is `20`.
  #[serde(rename = "replicationAckPropagationIntervalMsgCount")]
  replication_ack_propagation_interval_msg_count: Option<i64>,
  /// The Client Username the Replication Bridge uses to login to the Remote Message VPN on the Replication mate. The default value is `\"\"`.
  #[serde(rename = "replicationBridgeAuthenticationBasicClientUsername")]
  replication_bridge_authentication_basic_client_username: Option<String>,
  /// The password the Replication Bridge uses to login to the Remote Message VPN on the Replication mate. The default is to have no password. The default is to have no `replicationBridgeAuthenticationBasicPassword`.
  #[serde(rename = "replicationBridgeAuthenticationBasicPassword")]
  replication_bridge_authentication_basic_password: Option<String>,
  /// The PEM formatted content for the client certificate used by this bridge to login to the Remote Message VPN. It must consist of a private key and between one and three certificates comprising the certificate trust chain. The default value is `\"\"`. Available since 2.9.
  #[serde(rename = "replicationBridgeAuthenticationClientCertContent")]
  replication_bridge_authentication_client_cert_content: Option<String>,
  /// The password for the client certificate used by this bridge to login to the Remote Message VPN. The default value is `\"\"`. Available since 2.9.
  #[serde(rename = "replicationBridgeAuthenticationClientCertPassword")]
  replication_bridge_authentication_client_cert_password: Option<String>,
  /// The Authentication Scheme for the Replication Bridge in the Message VPN. The default value is `\"basic\"`. The allowed values and their meaning are:  <pre> \"basic\" - Basic Authentication Scheme (via username and password). \"client-certificate\" - Client Certificate Authentication Scheme (via certificate file or content). </pre> 
  #[serde(rename = "replicationBridgeAuthenticationScheme")]
  replication_bridge_authentication_scheme: Option<String>,
  /// Whether compression is used for the Replication Bridge. The default value is `false`.
  #[serde(rename = "replicationBridgeCompressedDataEnabled")]
  replication_bridge_compressed_data_enabled: Option<bool>,
  /// The size of the window used for guaranteed messages published to the Replication Bridge, in messages. The default value is `255`.
  #[serde(rename = "replicationBridgeEgressFlowWindowSize")]
  replication_bridge_egress_flow_window_size: Option<i64>,
  /// Number of seconds that must pass before retrying the Replication Bridge connection. The default value is `3`.
  #[serde(rename = "replicationBridgeRetryDelay")]
  replication_bridge_retry_delay: Option<i64>,
  /// Enable or disable use of TLS for the Replication Bridge connection. The default value is `false`.
  #[serde(rename = "replicationBridgeTlsEnabled")]
  replication_bridge_tls_enabled: Option<bool>,
  /// The Client Profile for the Unidirectional Replication Bridge. The Client Profile must exist in the local Message VPN, and it is used only for the TCP parameters. The default value is `\"#client-profile\"`.
  #[serde(rename = "replicationBridgeUnidirectionalClientProfileName")]
  replication_bridge_unidirectional_client_profile_name: Option<String>,
  /// Enable or disable the Replication feature for the Message VPN. The default value is `false`.
  #[serde(rename = "replicationEnabled")]
  replication_enabled: Option<bool>,
  /// The behavior to take when enabling the Replication feature for the Message VPN, depending on the existence of the Replication Queue. The default value is `\"fail-on-existing-queue\"`. The allowed values and their meaning are:  <pre> \"fail-on-existing-queue\" - The data replication queue must not already exist. \"force-use-existing-queue\" - The data replication queue must already exist. Any data messages on the Queue will be forwarded to interested applications. IMPORTANT: Before using this mode be certain that the messages are not stale or otherwise unsuitable to be forwarded. This mode can only be specified when the existing queue is configured the same as is currently specified under replication configuration otherwise the enabling of replication will fail. \"force-recreate-queue\" - The data replication queue must already exist. Any data messages on the Queue will be discarded. IMPORTANT: Before using this mode be certain that the messages on the existing data replication queue are not needed by interested applications. </pre> 
  #[serde(rename = "replicationEnabledQueueBehavior")]
  replication_enabled_queue_behavior: Option<String>,
  /// The maximum Message Spool usage by the Replication Bridge Queue (quota), in megabytes. The default value is `60000`.
  #[serde(rename = "replicationQueueMaxMsgSpoolUsage")]
  replication_queue_max_msg_spool_usage: Option<i64>,
  /// Assign the message discard behavior, that is the circumstances under which a negative acknowledgement (NACK) is sent to the Client on the Replication Bridge Queue discards. The default value is `true`.
  #[serde(rename = "replicationQueueRejectMsgToSenderOnDiscardEnabled")]
  replication_queue_reject_msg_to_sender_on_discard_enabled: Option<bool>,
  /// Enable or disable the synchronously replicated topics ineligible behavior of the Replication Bridge. If enabled and the synchronous replication becomes ineligible, guaranteed messages published to synchronously replicated topics will be rejected back to the sender as a negative acknowledgement (NACK). If disabled, the synchronous replication will revert to the asynchronous one. The default value is `false`.
  #[serde(rename = "replicationRejectMsgWhenSyncIneligibleEnabled")]
  replication_reject_msg_when_sync_ineligible_enabled: Option<bool>,
  /// The replication role for the Message VPN. The default value is `\"standby\"`. The allowed values and their meaning are:  <pre> \"active\" - Assume the Active role in Replication for the Message VPN. \"standby\" - Assume the Standby role in Replication for the Message VPN. </pre> 
  #[serde(rename = "replicationRole")]
  replication_role: Option<String>,
  /// The transaction replication mode for all transactions within the Message VPN. When mode is asynchronous, all transactions originated by clients are replicated to the standby site using the asynchronous replication. When mode is synchronous, all transactions originated by clients are replicated to the standby site using the synchronous replication. Changing this value during operation will not affect existing transactions, it is only used upon starting a transaction. The default value is `\"async\"`. The allowed values and their meaning are:  <pre> \"sync\" - Synchronous replication-mode. Published messages are acknowledged when they are spooled on the standby site. \"async\" - Asynchronous replication-mode. Published messages are acknowledged when they are spooled locally. </pre> 
  #[serde(rename = "replicationTransactionMode")]
  replication_transaction_mode: Option<String>,
  /// Enable or disable validation of the Common Name (CN) in the server certificate from the remote REST Consumer. If enabled, the Common Name is checked against the list of Trusted Common Names configured for the REST Consumer. The default value is `true`.
  #[serde(rename = "restTlsServerCertEnforceTrustedCommonNameEnabled")]
  rest_tls_server_cert_enforce_trusted_common_name_enabled: Option<bool>,
  /// The maximum depth for the server certificate from the remote REST Consumer chain. The depth of the chain is defined as the number of signing CA certificates that are present in the chain back to the trusted self-signed root CA certificate. The default value is `3`.
  #[serde(rename = "restTlsServerCertMaxChainDepth")]
  rest_tls_server_cert_max_chain_depth: Option<i64>,
  /// Enable or disable validation of the \"Not Before\" and \"Not After\" validity dates in the server certificate from the remote REST Consumer. When disabled, a certificate will be accepted even if the certificate is not valid according to the \"Not Before\" and \"Not After\" validity dates in the certificate. The default value is `true`.
  #[serde(rename = "restTlsServerCertValidateDateEnabled")]
  rest_tls_server_cert_validate_date_enabled: Option<bool>,
  /// Enable or disable \"admin client\" SEMP over the message bus commands for the current Message VPN. The default value is `false`.
  #[serde(rename = "sempOverMsgBusAdminClientEnabled")]
  semp_over_msg_bus_admin_client_enabled: Option<bool>,
  /// Enable or disable \"admin distributed-cache\" SEMP over the message bus commands for the current Message VPN. The default value is `false`.
  #[serde(rename = "sempOverMsgBusAdminDistributedCacheEnabled")]
  semp_over_msg_bus_admin_distributed_cache_enabled: Option<bool>,
  /// Enable or disable \"admin\" SEMP over the message bus commands for the current Message VPN. The default value is `false`.
  #[serde(rename = "sempOverMsgBusAdminEnabled")]
  semp_over_msg_bus_admin_enabled: Option<bool>,
  /// Enable or disable SEMP over the message bus for the current Message VPN. The default value is `true`.
  #[serde(rename = "sempOverMsgBusEnabled")]
  semp_over_msg_bus_enabled: Option<bool>,
  /// Enable or disable \"show\" SEMP over the message bus commands for the current Message VPN. The default value is `false`.
  #[serde(rename = "sempOverMsgBusShowEnabled")]
  semp_over_msg_bus_show_enabled: Option<bool>,
  /// The maximum number of AMQP client connections that can be simultaneously connected to the Message VPN. The default is the max value supported by the hardware. Available since 2.2.
  #[serde(rename = "serviceAmqpMaxConnectionCount")]
  service_amqp_max_connection_count: Option<i64>,
  /// Enable or disable the plain-text AMQP service in the Message VPN. Disabling causes clients connected to the corresponding listen-port to be disconnected. The default value is `false`. Available since 2.2.
  #[serde(rename = "serviceAmqpPlainTextEnabled")]
  service_amqp_plain_text_enabled: Option<bool>,
  /// The port number for plain-text AMQP clients that connect to the Message VPN. The default is to have no `serviceAmqpPlainTextListenPort`. Available since 2.2.
  #[serde(rename = "serviceAmqpPlainTextListenPort")]
  service_amqp_plain_text_listen_port: Option<i64>,
  /// Enable or disable the use of TLS for the AMQP service in the Message VPN. Disabling causes clients currently connected over TLS to be disconnected. The default value is `false`. Available since 2.2.
  #[serde(rename = "serviceAmqpTlsEnabled")]
  service_amqp_tls_enabled: Option<bool>,
  /// The port number for AMQP clients that connect to the Message VPN over TLS. The default is to have no `serviceAmqpTlsListenPort`. Available since 2.2.
  #[serde(rename = "serviceAmqpTlsListenPort")]
  service_amqp_tls_listen_port: Option<i64>,
  /// The maximum number of MQTT client connections that can be simultaneously connected to the Message VPN. The default is the max value supported by the hardware. Available since 2.1.
  #[serde(rename = "serviceMqttMaxConnectionCount")]
  service_mqtt_max_connection_count: Option<i64>,
  /// Enable or disable the plain-text MQTT service in the Message VPN. Disabling causes clients currently connected to be disconnected. The default value is `false`. Available since 2.1.
  #[serde(rename = "serviceMqttPlainTextEnabled")]
  service_mqtt_plain_text_enabled: Option<bool>,
  /// The port number for plain-text MQTT clients that connect to the Message VPN. The default value is `0`. Available since 2.1.
  #[serde(rename = "serviceMqttPlainTextListenPort")]
  service_mqtt_plain_text_listen_port: Option<i64>,
  /// Enable or disable the use of TLS for the MQTT service in the Message VPN. Disabling causes clients currently connected over TLS to be disconnected. The default value is `false`. Available since 2.1.
  #[serde(rename = "serviceMqttTlsEnabled")]
  service_mqtt_tls_enabled: Option<bool>,
  /// The port number for MQTT clients that connect to the Message VPN over TLS. The default value is `0`. Available since 2.1.
  #[serde(rename = "serviceMqttTlsListenPort")]
  service_mqtt_tls_listen_port: Option<i64>,
  /// Enable or disable the use of WebSocket over TLS for the MQTT service in the Message VPN. Disabling causes clients currently connected by WebSocket over TLS to be disconnected. The default value is `false`. Available since 2.1.
  #[serde(rename = "serviceMqttTlsWebSocketEnabled")]
  service_mqtt_tls_web_socket_enabled: Option<bool>,
  /// The port number for MQTT clients that connect to the Message VPN using WebSocket over TLS. The default value is `0`. Available since 2.1.
  #[serde(rename = "serviceMqttTlsWebSocketListenPort")]
  service_mqtt_tls_web_socket_listen_port: Option<i64>,
  /// Enable or disable the use of WebSocket for the MQTT service in the Message VPN. Disabling causes clients currently connected by WebSocket to be disconnected. The default value is `false`. Available since 2.1.
  #[serde(rename = "serviceMqttWebSocketEnabled")]
  service_mqtt_web_socket_enabled: Option<bool>,
  /// The port number for plain-text MQTT clients that connect to the Message VPN using WebSocket. The default value is `0`. Available since 2.1.
  #[serde(rename = "serviceMqttWebSocketListenPort")]
  service_mqtt_web_socket_listen_port: Option<i64>,
  /// The maximum number of REST incoming client connections that can be simultaneously connected to the Message VPN. The default is the max value supported by the hardware.
  #[serde(rename = "serviceRestIncomingMaxConnectionCount")]
  service_rest_incoming_max_connection_count: Option<i64>,
  /// Enable or disable the plain-text REST service for incoming clients in the Message VPN. Disabling causes clients currently connected to be disconnected. The default value is `false`.
  #[serde(rename = "serviceRestIncomingPlainTextEnabled")]
  service_rest_incoming_plain_text_enabled: Option<bool>,
  /// The port number for incoming plain-text REST clients that connect to the Message VPN. The default value is `0`.
  #[serde(rename = "serviceRestIncomingPlainTextListenPort")]
  service_rest_incoming_plain_text_listen_port: Option<i64>,
  /// Enable or disable the use of TLS for the REST service for incoming clients in the Message VPN. Disabling causes clients currently connected over TLS to be disconnected. The default value is `false`.
  #[serde(rename = "serviceRestIncomingTlsEnabled")]
  service_rest_incoming_tls_enabled: Option<bool>,
  /// The port number for incoming REST clients that connect to the Message VPN over TLS. The default value is `0`.
  #[serde(rename = "serviceRestIncomingTlsListenPort")]
  service_rest_incoming_tls_listen_port: Option<i64>,
  /// The REST service mode for incoming REST clients that connect to the Message VPN. The default value is `\"messaging\"`. The allowed values and their meaning are:  <pre> \"gateway\" - Act as a message gateway through which REST messages are propagated. \"messaging\" - Act as a message router on which REST messages are queued. </pre>  Available since 2.6.
  #[serde(rename = "serviceRestMode")]
  service_rest_mode: Option<String>,
  /// The maximum number of REST Consumer (outgoing) client connections that can be simultaneously connected to the Message VPN. The default varies by platform.
  #[serde(rename = "serviceRestOutgoingMaxConnectionCount")]
  service_rest_outgoing_max_connection_count: Option<i64>,
  /// The maximum number of SMF client connections that can be simultaneously connected to the Message VPN. The default is the max value supported by the hardware.
  #[serde(rename = "serviceSmfMaxConnectionCount")]
  service_smf_max_connection_count: Option<i64>,
  /// Enable or disable the plain-text SMF service in the Message VPN. Disabling causes clients currently connected to be disconnected. The default value is `true`.
  #[serde(rename = "serviceSmfPlainTextEnabled")]
  service_smf_plain_text_enabled: Option<bool>,
  /// Enable or disable the use of TLS for the SMF service in the Message VPN. Disabling causes clients currently connected over TLS to be disconnected. The default value is `true`.
  #[serde(rename = "serviceSmfTlsEnabled")]
  service_smf_tls_enabled: Option<bool>,
  /// The maximum number of Web Transport client connections that can be simultaneously connected to the Message VPN. The default is the max value supported by the hardware.
  #[serde(rename = "serviceWebMaxConnectionCount")]
  service_web_max_connection_count: Option<i64>,
  /// Enable or disable the plain-text Web Transport service in the Message VPN. Disabling causes clients currently connected to be disconnected. The default value is `true`.
  #[serde(rename = "serviceWebPlainTextEnabled")]
  service_web_plain_text_enabled: Option<bool>,
  /// Enable or disable the use of TLS for the Web Transport service in the Message VPN. Disabling causes clients currently connected over TLS to be disconnected. The default value is `true`.
  #[serde(rename = "serviceWebTlsEnabled")]
  service_web_tls_enabled: Option<bool>,
  /// Enable or disable the allowing of TLS SMF clients to downgrade their connections to plain-text connections. Changing this will not affect existing connections. The default value is `false`.
  #[serde(rename = "tlsAllowDowngradeToPlainTextEnabled")]
  tls_allow_downgrade_to_plain_text_enabled: Option<bool>
}

impl MsgVpn {
  pub fn new() -> MsgVpn {
    MsgVpn {
      authentication_basic_enabled: None,
      authentication_basic_profile_name: None,
      authentication_basic_radius_domain: None,
      authentication_basic_type: None,
      authentication_client_cert_allow_api_provided_username_enabled: None,
      authentication_client_cert_enabled: None,
      authentication_client_cert_max_chain_depth: None,
      authentication_client_cert_revocation_check_mode: None,
      authentication_client_cert_username_source: None,
      authentication_client_cert_validate_date_enabled: None,
      authentication_kerberos_allow_api_provided_username_enabled: None,
      authentication_kerberos_enabled: None,
      authorization_ldap_group_membership_attribute_name: None,
      authorization_profile_name: None,
      authorization_type: None,
      bridging_tls_server_cert_enforce_trusted_common_name_enabled: None,
      bridging_tls_server_cert_max_chain_depth: None,
      bridging_tls_server_cert_validate_date_enabled: None,
      distributed_cache_management_enabled: None,
      enabled: None,
      event_connection_count_threshold: None,
      event_egress_flow_count_threshold: None,
      event_egress_msg_rate_threshold: None,
      event_endpoint_count_threshold: None,
      event_ingress_flow_count_threshold: None,
      event_ingress_msg_rate_threshold: None,
      event_large_msg_threshold: None,
      event_log_tag: None,
      event_msg_spool_usage_threshold: None,
      event_publish_client_enabled: None,
      event_publish_msg_vpn_enabled: None,
      event_publish_subscription_mode: None,
      event_publish_topic_format_mqtt_enabled: None,
      event_publish_topic_format_smf_enabled: None,
      event_service_amqp_connection_count_threshold: None,
      event_service_mqtt_connection_count_threshold: None,
      event_service_rest_incoming_connection_count_threshold: None,
      event_service_smf_connection_count_threshold: None,
      event_service_web_connection_count_threshold: None,
      event_subscription_count_threshold: None,
      event_transacted_session_count_threshold: None,
      event_transaction_count_threshold: None,
      export_subscriptions_enabled: None,
      jndi_enabled: None,
      max_connection_count: None,
      max_egress_flow_count: None,
      max_endpoint_count: None,
      max_ingress_flow_count: None,
      max_msg_spool_usage: None,
      max_subscription_count: None,
      max_transacted_session_count: None,
      max_transaction_count: None,
      msg_vpn_name: None,
      replication_ack_propagation_interval_msg_count: None,
      replication_bridge_authentication_basic_client_username: None,
      replication_bridge_authentication_basic_password: None,
      replication_bridge_authentication_client_cert_content: None,
      replication_bridge_authentication_client_cert_password: None,
      replication_bridge_authentication_scheme: None,
      replication_bridge_compressed_data_enabled: None,
      replication_bridge_egress_flow_window_size: None,
      replication_bridge_retry_delay: None,
      replication_bridge_tls_enabled: None,
      replication_bridge_unidirectional_client_profile_name: None,
      replication_enabled: None,
      replication_enabled_queue_behavior: None,
      replication_queue_max_msg_spool_usage: None,
      replication_queue_reject_msg_to_sender_on_discard_enabled: None,
      replication_reject_msg_when_sync_ineligible_enabled: None,
      replication_role: None,
      replication_transaction_mode: None,
      rest_tls_server_cert_enforce_trusted_common_name_enabled: None,
      rest_tls_server_cert_max_chain_depth: None,
      rest_tls_server_cert_validate_date_enabled: None,
      semp_over_msg_bus_admin_client_enabled: None,
      semp_over_msg_bus_admin_distributed_cache_enabled: None,
      semp_over_msg_bus_admin_enabled: None,
      semp_over_msg_bus_enabled: None,
      semp_over_msg_bus_show_enabled: None,
      service_amqp_max_connection_count: None,
      service_amqp_plain_text_enabled: None,
      service_amqp_plain_text_listen_port: None,
      service_amqp_tls_enabled: None,
      service_amqp_tls_listen_port: None,
      service_mqtt_max_connection_count: None,
      service_mqtt_plain_text_enabled: None,
      service_mqtt_plain_text_listen_port: None,
      service_mqtt_tls_enabled: None,
      service_mqtt_tls_listen_port: None,
      service_mqtt_tls_web_socket_enabled: None,
      service_mqtt_tls_web_socket_listen_port: None,
      service_mqtt_web_socket_enabled: None,
      service_mqtt_web_socket_listen_port: None,
      service_rest_incoming_max_connection_count: None,
      service_rest_incoming_plain_text_enabled: None,
      service_rest_incoming_plain_text_listen_port: None,
      service_rest_incoming_tls_enabled: None,
      service_rest_incoming_tls_listen_port: None,
      service_rest_mode: None,
      service_rest_outgoing_max_connection_count: None,
      service_smf_max_connection_count: None,
      service_smf_plain_text_enabled: None,
      service_smf_tls_enabled: None,
      service_web_max_connection_count: None,
      service_web_plain_text_enabled: None,
      service_web_tls_enabled: None,
      tls_allow_downgrade_to_plain_text_enabled: None
    }
  }

  pub fn set_authentication_basic_enabled(&mut self, authentication_basic_enabled: bool) {
    self.authentication_basic_enabled = Some(authentication_basic_enabled);
  }

  pub fn with_authentication_basic_enabled(mut self, authentication_basic_enabled: bool) -> MsgVpn {
    self.authentication_basic_enabled = Some(authentication_basic_enabled);
    self
  }

  pub fn authentication_basic_enabled(&self) -> Option<&bool> {
    self.authentication_basic_enabled.as_ref()
  }

  pub fn reset_authentication_basic_enabled(&mut self) {
    self.authentication_basic_enabled = None;
  }

  pub fn set_authentication_basic_profile_name(&mut self, authentication_basic_profile_name: String) {
    self.authentication_basic_profile_name = Some(authentication_basic_profile_name);
  }

  pub fn with_authentication_basic_profile_name(mut self, authentication_basic_profile_name: String) -> MsgVpn {
    self.authentication_basic_profile_name = Some(authentication_basic_profile_name);
    self
  }

  pub fn authentication_basic_profile_name(&self) -> Option<&String> {
    self.authentication_basic_profile_name.as_ref()
  }

  pub fn reset_authentication_basic_profile_name(&mut self) {
    self.authentication_basic_profile_name = None;
  }

  pub fn set_authentication_basic_radius_domain(&mut self, authentication_basic_radius_domain: String) {
    self.authentication_basic_radius_domain = Some(authentication_basic_radius_domain);
  }

  pub fn with_authentication_basic_radius_domain(mut self, authentication_basic_radius_domain: String) -> MsgVpn {
    self.authentication_basic_radius_domain = Some(authentication_basic_radius_domain);
    self
  }

  pub fn authentication_basic_radius_domain(&self) -> Option<&String> {
    self.authentication_basic_radius_domain.as_ref()
  }

  pub fn reset_authentication_basic_radius_domain(&mut self) {
    self.authentication_basic_radius_domain = None;
  }

  pub fn set_authentication_basic_type(&mut self, authentication_basic_type: String) {
    self.authentication_basic_type = Some(authentication_basic_type);
  }

  pub fn with_authentication_basic_type(mut self, authentication_basic_type: String) -> MsgVpn {
    self.authentication_basic_type = Some(authentication_basic_type);
    self
  }

  pub fn authentication_basic_type(&self) -> Option<&String> {
    self.authentication_basic_type.as_ref()
  }

  pub fn reset_authentication_basic_type(&mut self) {
    self.authentication_basic_type = None;
  }

  pub fn set_authentication_client_cert_allow_api_provided_username_enabled(&mut self, authentication_client_cert_allow_api_provided_username_enabled: bool) {
    self.authentication_client_cert_allow_api_provided_username_enabled = Some(authentication_client_cert_allow_api_provided_username_enabled);
  }

  pub fn with_authentication_client_cert_allow_api_provided_username_enabled(mut self, authentication_client_cert_allow_api_provided_username_enabled: bool) -> MsgVpn {
    self.authentication_client_cert_allow_api_provided_username_enabled = Some(authentication_client_cert_allow_api_provided_username_enabled);
    self
  }

  pub fn authentication_client_cert_allow_api_provided_username_enabled(&self) -> Option<&bool> {
    self.authentication_client_cert_allow_api_provided_username_enabled.as_ref()
  }

  pub fn reset_authentication_client_cert_allow_api_provided_username_enabled(&mut self) {
    self.authentication_client_cert_allow_api_provided_username_enabled = None;
  }

  pub fn set_authentication_client_cert_enabled(&mut self, authentication_client_cert_enabled: bool) {
    self.authentication_client_cert_enabled = Some(authentication_client_cert_enabled);
  }

  pub fn with_authentication_client_cert_enabled(mut self, authentication_client_cert_enabled: bool) -> MsgVpn {
    self.authentication_client_cert_enabled = Some(authentication_client_cert_enabled);
    self
  }

  pub fn authentication_client_cert_enabled(&self) -> Option<&bool> {
    self.authentication_client_cert_enabled.as_ref()
  }

  pub fn reset_authentication_client_cert_enabled(&mut self) {
    self.authentication_client_cert_enabled = None;
  }

  pub fn set_authentication_client_cert_max_chain_depth(&mut self, authentication_client_cert_max_chain_depth: i64) {
    self.authentication_client_cert_max_chain_depth = Some(authentication_client_cert_max_chain_depth);
  }

  pub fn with_authentication_client_cert_max_chain_depth(mut self, authentication_client_cert_max_chain_depth: i64) -> MsgVpn {
    self.authentication_client_cert_max_chain_depth = Some(authentication_client_cert_max_chain_depth);
    self
  }

  pub fn authentication_client_cert_max_chain_depth(&self) -> Option<&i64> {
    self.authentication_client_cert_max_chain_depth.as_ref()
  }

  pub fn reset_authentication_client_cert_max_chain_depth(&mut self) {
    self.authentication_client_cert_max_chain_depth = None;
  }

  pub fn set_authentication_client_cert_revocation_check_mode(&mut self, authentication_client_cert_revocation_check_mode: String) {
    self.authentication_client_cert_revocation_check_mode = Some(authentication_client_cert_revocation_check_mode);
  }

  pub fn with_authentication_client_cert_revocation_check_mode(mut self, authentication_client_cert_revocation_check_mode: String) -> MsgVpn {
    self.authentication_client_cert_revocation_check_mode = Some(authentication_client_cert_revocation_check_mode);
    self
  }

  pub fn authentication_client_cert_revocation_check_mode(&self) -> Option<&String> {
    self.authentication_client_cert_revocation_check_mode.as_ref()
  }

  pub fn reset_authentication_client_cert_revocation_check_mode(&mut self) {
    self.authentication_client_cert_revocation_check_mode = None;
  }

  pub fn set_authentication_client_cert_username_source(&mut self, authentication_client_cert_username_source: String) {
    self.authentication_client_cert_username_source = Some(authentication_client_cert_username_source);
  }

  pub fn with_authentication_client_cert_username_source(mut self, authentication_client_cert_username_source: String) -> MsgVpn {
    self.authentication_client_cert_username_source = Some(authentication_client_cert_username_source);
    self
  }

  pub fn authentication_client_cert_username_source(&self) -> Option<&String> {
    self.authentication_client_cert_username_source.as_ref()
  }

  pub fn reset_authentication_client_cert_username_source(&mut self) {
    self.authentication_client_cert_username_source = None;
  }

  pub fn set_authentication_client_cert_validate_date_enabled(&mut self, authentication_client_cert_validate_date_enabled: bool) {
    self.authentication_client_cert_validate_date_enabled = Some(authentication_client_cert_validate_date_enabled);
  }

  pub fn with_authentication_client_cert_validate_date_enabled(mut self, authentication_client_cert_validate_date_enabled: bool) -> MsgVpn {
    self.authentication_client_cert_validate_date_enabled = Some(authentication_client_cert_validate_date_enabled);
    self
  }

  pub fn authentication_client_cert_validate_date_enabled(&self) -> Option<&bool> {
    self.authentication_client_cert_validate_date_enabled.as_ref()
  }

  pub fn reset_authentication_client_cert_validate_date_enabled(&mut self) {
    self.authentication_client_cert_validate_date_enabled = None;
  }

  pub fn set_authentication_kerberos_allow_api_provided_username_enabled(&mut self, authentication_kerberos_allow_api_provided_username_enabled: bool) {
    self.authentication_kerberos_allow_api_provided_username_enabled = Some(authentication_kerberos_allow_api_provided_username_enabled);
  }

  pub fn with_authentication_kerberos_allow_api_provided_username_enabled(mut self, authentication_kerberos_allow_api_provided_username_enabled: bool) -> MsgVpn {
    self.authentication_kerberos_allow_api_provided_username_enabled = Some(authentication_kerberos_allow_api_provided_username_enabled);
    self
  }

  pub fn authentication_kerberos_allow_api_provided_username_enabled(&self) -> Option<&bool> {
    self.authentication_kerberos_allow_api_provided_username_enabled.as_ref()
  }

  pub fn reset_authentication_kerberos_allow_api_provided_username_enabled(&mut self) {
    self.authentication_kerberos_allow_api_provided_username_enabled = None;
  }

  pub fn set_authentication_kerberos_enabled(&mut self, authentication_kerberos_enabled: bool) {
    self.authentication_kerberos_enabled = Some(authentication_kerberos_enabled);
  }

  pub fn with_authentication_kerberos_enabled(mut self, authentication_kerberos_enabled: bool) -> MsgVpn {
    self.authentication_kerberos_enabled = Some(authentication_kerberos_enabled);
    self
  }

  pub fn authentication_kerberos_enabled(&self) -> Option<&bool> {
    self.authentication_kerberos_enabled.as_ref()
  }

  pub fn reset_authentication_kerberos_enabled(&mut self) {
    self.authentication_kerberos_enabled = None;
  }

  pub fn set_authorization_ldap_group_membership_attribute_name(&mut self, authorization_ldap_group_membership_attribute_name: String) {
    self.authorization_ldap_group_membership_attribute_name = Some(authorization_ldap_group_membership_attribute_name);
  }

  pub fn with_authorization_ldap_group_membership_attribute_name(mut self, authorization_ldap_group_membership_attribute_name: String) -> MsgVpn {
    self.authorization_ldap_group_membership_attribute_name = Some(authorization_ldap_group_membership_attribute_name);
    self
  }

  pub fn authorization_ldap_group_membership_attribute_name(&self) -> Option<&String> {
    self.authorization_ldap_group_membership_attribute_name.as_ref()
  }

  pub fn reset_authorization_ldap_group_membership_attribute_name(&mut self) {
    self.authorization_ldap_group_membership_attribute_name = None;
  }

  pub fn set_authorization_profile_name(&mut self, authorization_profile_name: String) {
    self.authorization_profile_name = Some(authorization_profile_name);
  }

  pub fn with_authorization_profile_name(mut self, authorization_profile_name: String) -> MsgVpn {
    self.authorization_profile_name = Some(authorization_profile_name);
    self
  }

  pub fn authorization_profile_name(&self) -> Option<&String> {
    self.authorization_profile_name.as_ref()
  }

  pub fn reset_authorization_profile_name(&mut self) {
    self.authorization_profile_name = None;
  }

  pub fn set_authorization_type(&mut self, authorization_type: String) {
    self.authorization_type = Some(authorization_type);
  }

  pub fn with_authorization_type(mut self, authorization_type: String) -> MsgVpn {
    self.authorization_type = Some(authorization_type);
    self
  }

  pub fn authorization_type(&self) -> Option<&String> {
    self.authorization_type.as_ref()
  }

  pub fn reset_authorization_type(&mut self) {
    self.authorization_type = None;
  }

  pub fn set_bridging_tls_server_cert_enforce_trusted_common_name_enabled(&mut self, bridging_tls_server_cert_enforce_trusted_common_name_enabled: bool) {
    self.bridging_tls_server_cert_enforce_trusted_common_name_enabled = Some(bridging_tls_server_cert_enforce_trusted_common_name_enabled);
  }

  pub fn with_bridging_tls_server_cert_enforce_trusted_common_name_enabled(mut self, bridging_tls_server_cert_enforce_trusted_common_name_enabled: bool) -> MsgVpn {
    self.bridging_tls_server_cert_enforce_trusted_common_name_enabled = Some(bridging_tls_server_cert_enforce_trusted_common_name_enabled);
    self
  }

  pub fn bridging_tls_server_cert_enforce_trusted_common_name_enabled(&self) -> Option<&bool> {
    self.bridging_tls_server_cert_enforce_trusted_common_name_enabled.as_ref()
  }

  pub fn reset_bridging_tls_server_cert_enforce_trusted_common_name_enabled(&mut self) {
    self.bridging_tls_server_cert_enforce_trusted_common_name_enabled = None;
  }

  pub fn set_bridging_tls_server_cert_max_chain_depth(&mut self, bridging_tls_server_cert_max_chain_depth: i64) {
    self.bridging_tls_server_cert_max_chain_depth = Some(bridging_tls_server_cert_max_chain_depth);
  }

  pub fn with_bridging_tls_server_cert_max_chain_depth(mut self, bridging_tls_server_cert_max_chain_depth: i64) -> MsgVpn {
    self.bridging_tls_server_cert_max_chain_depth = Some(bridging_tls_server_cert_max_chain_depth);
    self
  }

  pub fn bridging_tls_server_cert_max_chain_depth(&self) -> Option<&i64> {
    self.bridging_tls_server_cert_max_chain_depth.as_ref()
  }

  pub fn reset_bridging_tls_server_cert_max_chain_depth(&mut self) {
    self.bridging_tls_server_cert_max_chain_depth = None;
  }

  pub fn set_bridging_tls_server_cert_validate_date_enabled(&mut self, bridging_tls_server_cert_validate_date_enabled: bool) {
    self.bridging_tls_server_cert_validate_date_enabled = Some(bridging_tls_server_cert_validate_date_enabled);
  }

  pub fn with_bridging_tls_server_cert_validate_date_enabled(mut self, bridging_tls_server_cert_validate_date_enabled: bool) -> MsgVpn {
    self.bridging_tls_server_cert_validate_date_enabled = Some(bridging_tls_server_cert_validate_date_enabled);
    self
  }

  pub fn bridging_tls_server_cert_validate_date_enabled(&self) -> Option<&bool> {
    self.bridging_tls_server_cert_validate_date_enabled.as_ref()
  }

  pub fn reset_bridging_tls_server_cert_validate_date_enabled(&mut self) {
    self.bridging_tls_server_cert_validate_date_enabled = None;
  }

  pub fn set_distributed_cache_management_enabled(&mut self, distributed_cache_management_enabled: bool) {
    self.distributed_cache_management_enabled = Some(distributed_cache_management_enabled);
  }

  pub fn with_distributed_cache_management_enabled(mut self, distributed_cache_management_enabled: bool) -> MsgVpn {
    self.distributed_cache_management_enabled = Some(distributed_cache_management_enabled);
    self
  }

  pub fn distributed_cache_management_enabled(&self) -> Option<&bool> {
    self.distributed_cache_management_enabled.as_ref()
  }

  pub fn reset_distributed_cache_management_enabled(&mut self) {
    self.distributed_cache_management_enabled = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> MsgVpn {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_event_connection_count_threshold(&mut self, event_connection_count_threshold: ::models::EventThreshold) {
    self.event_connection_count_threshold = Some(event_connection_count_threshold);
  }

  pub fn with_event_connection_count_threshold(mut self, event_connection_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_connection_count_threshold = Some(event_connection_count_threshold);
    self
  }

  pub fn event_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_connection_count_threshold.as_ref()
  }

  pub fn reset_event_connection_count_threshold(&mut self) {
    self.event_connection_count_threshold = None;
  }

  pub fn set_event_egress_flow_count_threshold(&mut self, event_egress_flow_count_threshold: ::models::EventThreshold) {
    self.event_egress_flow_count_threshold = Some(event_egress_flow_count_threshold);
  }

  pub fn with_event_egress_flow_count_threshold(mut self, event_egress_flow_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_egress_flow_count_threshold = Some(event_egress_flow_count_threshold);
    self
  }

  pub fn event_egress_flow_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_egress_flow_count_threshold.as_ref()
  }

  pub fn reset_event_egress_flow_count_threshold(&mut self) {
    self.event_egress_flow_count_threshold = None;
  }

  pub fn set_event_egress_msg_rate_threshold(&mut self, event_egress_msg_rate_threshold: ::models::EventThresholdByValue) {
    self.event_egress_msg_rate_threshold = Some(event_egress_msg_rate_threshold);
  }

  pub fn with_event_egress_msg_rate_threshold(mut self, event_egress_msg_rate_threshold: ::models::EventThresholdByValue) -> MsgVpn {
    self.event_egress_msg_rate_threshold = Some(event_egress_msg_rate_threshold);
    self
  }

  pub fn event_egress_msg_rate_threshold(&self) -> Option<&::models::EventThresholdByValue> {
    self.event_egress_msg_rate_threshold.as_ref()
  }

  pub fn reset_event_egress_msg_rate_threshold(&mut self) {
    self.event_egress_msg_rate_threshold = None;
  }

  pub fn set_event_endpoint_count_threshold(&mut self, event_endpoint_count_threshold: ::models::EventThreshold) {
    self.event_endpoint_count_threshold = Some(event_endpoint_count_threshold);
  }

  pub fn with_event_endpoint_count_threshold(mut self, event_endpoint_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_endpoint_count_threshold = Some(event_endpoint_count_threshold);
    self
  }

  pub fn event_endpoint_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_endpoint_count_threshold.as_ref()
  }

  pub fn reset_event_endpoint_count_threshold(&mut self) {
    self.event_endpoint_count_threshold = None;
  }

  pub fn set_event_ingress_flow_count_threshold(&mut self, event_ingress_flow_count_threshold: ::models::EventThreshold) {
    self.event_ingress_flow_count_threshold = Some(event_ingress_flow_count_threshold);
  }

  pub fn with_event_ingress_flow_count_threshold(mut self, event_ingress_flow_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_ingress_flow_count_threshold = Some(event_ingress_flow_count_threshold);
    self
  }

  pub fn event_ingress_flow_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_ingress_flow_count_threshold.as_ref()
  }

  pub fn reset_event_ingress_flow_count_threshold(&mut self) {
    self.event_ingress_flow_count_threshold = None;
  }

  pub fn set_event_ingress_msg_rate_threshold(&mut self, event_ingress_msg_rate_threshold: ::models::EventThresholdByValue) {
    self.event_ingress_msg_rate_threshold = Some(event_ingress_msg_rate_threshold);
  }

  pub fn with_event_ingress_msg_rate_threshold(mut self, event_ingress_msg_rate_threshold: ::models::EventThresholdByValue) -> MsgVpn {
    self.event_ingress_msg_rate_threshold = Some(event_ingress_msg_rate_threshold);
    self
  }

  pub fn event_ingress_msg_rate_threshold(&self) -> Option<&::models::EventThresholdByValue> {
    self.event_ingress_msg_rate_threshold.as_ref()
  }

  pub fn reset_event_ingress_msg_rate_threshold(&mut self) {
    self.event_ingress_msg_rate_threshold = None;
  }

  pub fn set_event_large_msg_threshold(&mut self, event_large_msg_threshold: i64) {
    self.event_large_msg_threshold = Some(event_large_msg_threshold);
  }

  pub fn with_event_large_msg_threshold(mut self, event_large_msg_threshold: i64) -> MsgVpn {
    self.event_large_msg_threshold = Some(event_large_msg_threshold);
    self
  }

  pub fn event_large_msg_threshold(&self) -> Option<&i64> {
    self.event_large_msg_threshold.as_ref()
  }

  pub fn reset_event_large_msg_threshold(&mut self) {
    self.event_large_msg_threshold = None;
  }

  pub fn set_event_log_tag(&mut self, event_log_tag: String) {
    self.event_log_tag = Some(event_log_tag);
  }

  pub fn with_event_log_tag(mut self, event_log_tag: String) -> MsgVpn {
    self.event_log_tag = Some(event_log_tag);
    self
  }

  pub fn event_log_tag(&self) -> Option<&String> {
    self.event_log_tag.as_ref()
  }

  pub fn reset_event_log_tag(&mut self) {
    self.event_log_tag = None;
  }

  pub fn set_event_msg_spool_usage_threshold(&mut self, event_msg_spool_usage_threshold: ::models::EventThreshold) {
    self.event_msg_spool_usage_threshold = Some(event_msg_spool_usage_threshold);
  }

  pub fn with_event_msg_spool_usage_threshold(mut self, event_msg_spool_usage_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_msg_spool_usage_threshold = Some(event_msg_spool_usage_threshold);
    self
  }

  pub fn event_msg_spool_usage_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_msg_spool_usage_threshold.as_ref()
  }

  pub fn reset_event_msg_spool_usage_threshold(&mut self) {
    self.event_msg_spool_usage_threshold = None;
  }

  pub fn set_event_publish_client_enabled(&mut self, event_publish_client_enabled: bool) {
    self.event_publish_client_enabled = Some(event_publish_client_enabled);
  }

  pub fn with_event_publish_client_enabled(mut self, event_publish_client_enabled: bool) -> MsgVpn {
    self.event_publish_client_enabled = Some(event_publish_client_enabled);
    self
  }

  pub fn event_publish_client_enabled(&self) -> Option<&bool> {
    self.event_publish_client_enabled.as_ref()
  }

  pub fn reset_event_publish_client_enabled(&mut self) {
    self.event_publish_client_enabled = None;
  }

  pub fn set_event_publish_msg_vpn_enabled(&mut self, event_publish_msg_vpn_enabled: bool) {
    self.event_publish_msg_vpn_enabled = Some(event_publish_msg_vpn_enabled);
  }

  pub fn with_event_publish_msg_vpn_enabled(mut self, event_publish_msg_vpn_enabled: bool) -> MsgVpn {
    self.event_publish_msg_vpn_enabled = Some(event_publish_msg_vpn_enabled);
    self
  }

  pub fn event_publish_msg_vpn_enabled(&self) -> Option<&bool> {
    self.event_publish_msg_vpn_enabled.as_ref()
  }

  pub fn reset_event_publish_msg_vpn_enabled(&mut self) {
    self.event_publish_msg_vpn_enabled = None;
  }

  pub fn set_event_publish_subscription_mode(&mut self, event_publish_subscription_mode: String) {
    self.event_publish_subscription_mode = Some(event_publish_subscription_mode);
  }

  pub fn with_event_publish_subscription_mode(mut self, event_publish_subscription_mode: String) -> MsgVpn {
    self.event_publish_subscription_mode = Some(event_publish_subscription_mode);
    self
  }

  pub fn event_publish_subscription_mode(&self) -> Option<&String> {
    self.event_publish_subscription_mode.as_ref()
  }

  pub fn reset_event_publish_subscription_mode(&mut self) {
    self.event_publish_subscription_mode = None;
  }

  pub fn set_event_publish_topic_format_mqtt_enabled(&mut self, event_publish_topic_format_mqtt_enabled: bool) {
    self.event_publish_topic_format_mqtt_enabled = Some(event_publish_topic_format_mqtt_enabled);
  }

  pub fn with_event_publish_topic_format_mqtt_enabled(mut self, event_publish_topic_format_mqtt_enabled: bool) -> MsgVpn {
    self.event_publish_topic_format_mqtt_enabled = Some(event_publish_topic_format_mqtt_enabled);
    self
  }

  pub fn event_publish_topic_format_mqtt_enabled(&self) -> Option<&bool> {
    self.event_publish_topic_format_mqtt_enabled.as_ref()
  }

  pub fn reset_event_publish_topic_format_mqtt_enabled(&mut self) {
    self.event_publish_topic_format_mqtt_enabled = None;
  }

  pub fn set_event_publish_topic_format_smf_enabled(&mut self, event_publish_topic_format_smf_enabled: bool) {
    self.event_publish_topic_format_smf_enabled = Some(event_publish_topic_format_smf_enabled);
  }

  pub fn with_event_publish_topic_format_smf_enabled(mut self, event_publish_topic_format_smf_enabled: bool) -> MsgVpn {
    self.event_publish_topic_format_smf_enabled = Some(event_publish_topic_format_smf_enabled);
    self
  }

  pub fn event_publish_topic_format_smf_enabled(&self) -> Option<&bool> {
    self.event_publish_topic_format_smf_enabled.as_ref()
  }

  pub fn reset_event_publish_topic_format_smf_enabled(&mut self) {
    self.event_publish_topic_format_smf_enabled = None;
  }

  pub fn set_event_service_amqp_connection_count_threshold(&mut self, event_service_amqp_connection_count_threshold: ::models::EventThreshold) {
    self.event_service_amqp_connection_count_threshold = Some(event_service_amqp_connection_count_threshold);
  }

  pub fn with_event_service_amqp_connection_count_threshold(mut self, event_service_amqp_connection_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_service_amqp_connection_count_threshold = Some(event_service_amqp_connection_count_threshold);
    self
  }

  pub fn event_service_amqp_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_service_amqp_connection_count_threshold.as_ref()
  }

  pub fn reset_event_service_amqp_connection_count_threshold(&mut self) {
    self.event_service_amqp_connection_count_threshold = None;
  }

  pub fn set_event_service_mqtt_connection_count_threshold(&mut self, event_service_mqtt_connection_count_threshold: ::models::EventThreshold) {
    self.event_service_mqtt_connection_count_threshold = Some(event_service_mqtt_connection_count_threshold);
  }

  pub fn with_event_service_mqtt_connection_count_threshold(mut self, event_service_mqtt_connection_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_service_mqtt_connection_count_threshold = Some(event_service_mqtt_connection_count_threshold);
    self
  }

  pub fn event_service_mqtt_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_service_mqtt_connection_count_threshold.as_ref()
  }

  pub fn reset_event_service_mqtt_connection_count_threshold(&mut self) {
    self.event_service_mqtt_connection_count_threshold = None;
  }

  pub fn set_event_service_rest_incoming_connection_count_threshold(&mut self, event_service_rest_incoming_connection_count_threshold: ::models::EventThreshold) {
    self.event_service_rest_incoming_connection_count_threshold = Some(event_service_rest_incoming_connection_count_threshold);
  }

  pub fn with_event_service_rest_incoming_connection_count_threshold(mut self, event_service_rest_incoming_connection_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_service_rest_incoming_connection_count_threshold = Some(event_service_rest_incoming_connection_count_threshold);
    self
  }

  pub fn event_service_rest_incoming_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_service_rest_incoming_connection_count_threshold.as_ref()
  }

  pub fn reset_event_service_rest_incoming_connection_count_threshold(&mut self) {
    self.event_service_rest_incoming_connection_count_threshold = None;
  }

  pub fn set_event_service_smf_connection_count_threshold(&mut self, event_service_smf_connection_count_threshold: ::models::EventThreshold) {
    self.event_service_smf_connection_count_threshold = Some(event_service_smf_connection_count_threshold);
  }

  pub fn with_event_service_smf_connection_count_threshold(mut self, event_service_smf_connection_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_service_smf_connection_count_threshold = Some(event_service_smf_connection_count_threshold);
    self
  }

  pub fn event_service_smf_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_service_smf_connection_count_threshold.as_ref()
  }

  pub fn reset_event_service_smf_connection_count_threshold(&mut self) {
    self.event_service_smf_connection_count_threshold = None;
  }

  pub fn set_event_service_web_connection_count_threshold(&mut self, event_service_web_connection_count_threshold: ::models::EventThreshold) {
    self.event_service_web_connection_count_threshold = Some(event_service_web_connection_count_threshold);
  }

  pub fn with_event_service_web_connection_count_threshold(mut self, event_service_web_connection_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_service_web_connection_count_threshold = Some(event_service_web_connection_count_threshold);
    self
  }

  pub fn event_service_web_connection_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_service_web_connection_count_threshold.as_ref()
  }

  pub fn reset_event_service_web_connection_count_threshold(&mut self) {
    self.event_service_web_connection_count_threshold = None;
  }

  pub fn set_event_subscription_count_threshold(&mut self, event_subscription_count_threshold: ::models::EventThreshold) {
    self.event_subscription_count_threshold = Some(event_subscription_count_threshold);
  }

  pub fn with_event_subscription_count_threshold(mut self, event_subscription_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_subscription_count_threshold = Some(event_subscription_count_threshold);
    self
  }

  pub fn event_subscription_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_subscription_count_threshold.as_ref()
  }

  pub fn reset_event_subscription_count_threshold(&mut self) {
    self.event_subscription_count_threshold = None;
  }

  pub fn set_event_transacted_session_count_threshold(&mut self, event_transacted_session_count_threshold: ::models::EventThreshold) {
    self.event_transacted_session_count_threshold = Some(event_transacted_session_count_threshold);
  }

  pub fn with_event_transacted_session_count_threshold(mut self, event_transacted_session_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_transacted_session_count_threshold = Some(event_transacted_session_count_threshold);
    self
  }

  pub fn event_transacted_session_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_transacted_session_count_threshold.as_ref()
  }

  pub fn reset_event_transacted_session_count_threshold(&mut self) {
    self.event_transacted_session_count_threshold = None;
  }

  pub fn set_event_transaction_count_threshold(&mut self, event_transaction_count_threshold: ::models::EventThreshold) {
    self.event_transaction_count_threshold = Some(event_transaction_count_threshold);
  }

  pub fn with_event_transaction_count_threshold(mut self, event_transaction_count_threshold: ::models::EventThreshold) -> MsgVpn {
    self.event_transaction_count_threshold = Some(event_transaction_count_threshold);
    self
  }

  pub fn event_transaction_count_threshold(&self) -> Option<&::models::EventThreshold> {
    self.event_transaction_count_threshold.as_ref()
  }

  pub fn reset_event_transaction_count_threshold(&mut self) {
    self.event_transaction_count_threshold = None;
  }

  pub fn set_export_subscriptions_enabled(&mut self, export_subscriptions_enabled: bool) {
    self.export_subscriptions_enabled = Some(export_subscriptions_enabled);
  }

  pub fn with_export_subscriptions_enabled(mut self, export_subscriptions_enabled: bool) -> MsgVpn {
    self.export_subscriptions_enabled = Some(export_subscriptions_enabled);
    self
  }

  pub fn export_subscriptions_enabled(&self) -> Option<&bool> {
    self.export_subscriptions_enabled.as_ref()
  }

  pub fn reset_export_subscriptions_enabled(&mut self) {
    self.export_subscriptions_enabled = None;
  }

  pub fn set_jndi_enabled(&mut self, jndi_enabled: bool) {
    self.jndi_enabled = Some(jndi_enabled);
  }

  pub fn with_jndi_enabled(mut self, jndi_enabled: bool) -> MsgVpn {
    self.jndi_enabled = Some(jndi_enabled);
    self
  }

  pub fn jndi_enabled(&self) -> Option<&bool> {
    self.jndi_enabled.as_ref()
  }

  pub fn reset_jndi_enabled(&mut self) {
    self.jndi_enabled = None;
  }

  pub fn set_max_connection_count(&mut self, max_connection_count: i64) {
    self.max_connection_count = Some(max_connection_count);
  }

  pub fn with_max_connection_count(mut self, max_connection_count: i64) -> MsgVpn {
    self.max_connection_count = Some(max_connection_count);
    self
  }

  pub fn max_connection_count(&self) -> Option<&i64> {
    self.max_connection_count.as_ref()
  }

  pub fn reset_max_connection_count(&mut self) {
    self.max_connection_count = None;
  }

  pub fn set_max_egress_flow_count(&mut self, max_egress_flow_count: i64) {
    self.max_egress_flow_count = Some(max_egress_flow_count);
  }

  pub fn with_max_egress_flow_count(mut self, max_egress_flow_count: i64) -> MsgVpn {
    self.max_egress_flow_count = Some(max_egress_flow_count);
    self
  }

  pub fn max_egress_flow_count(&self) -> Option<&i64> {
    self.max_egress_flow_count.as_ref()
  }

  pub fn reset_max_egress_flow_count(&mut self) {
    self.max_egress_flow_count = None;
  }

  pub fn set_max_endpoint_count(&mut self, max_endpoint_count: i64) {
    self.max_endpoint_count = Some(max_endpoint_count);
  }

  pub fn with_max_endpoint_count(mut self, max_endpoint_count: i64) -> MsgVpn {
    self.max_endpoint_count = Some(max_endpoint_count);
    self
  }

  pub fn max_endpoint_count(&self) -> Option<&i64> {
    self.max_endpoint_count.as_ref()
  }

  pub fn reset_max_endpoint_count(&mut self) {
    self.max_endpoint_count = None;
  }

  pub fn set_max_ingress_flow_count(&mut self, max_ingress_flow_count: i64) {
    self.max_ingress_flow_count = Some(max_ingress_flow_count);
  }

  pub fn with_max_ingress_flow_count(mut self, max_ingress_flow_count: i64) -> MsgVpn {
    self.max_ingress_flow_count = Some(max_ingress_flow_count);
    self
  }

  pub fn max_ingress_flow_count(&self) -> Option<&i64> {
    self.max_ingress_flow_count.as_ref()
  }

  pub fn reset_max_ingress_flow_count(&mut self) {
    self.max_ingress_flow_count = None;
  }

  pub fn set_max_msg_spool_usage(&mut self, max_msg_spool_usage: i64) {
    self.max_msg_spool_usage = Some(max_msg_spool_usage);
  }

  pub fn with_max_msg_spool_usage(mut self, max_msg_spool_usage: i64) -> MsgVpn {
    self.max_msg_spool_usage = Some(max_msg_spool_usage);
    self
  }

  pub fn max_msg_spool_usage(&self) -> Option<&i64> {
    self.max_msg_spool_usage.as_ref()
  }

  pub fn reset_max_msg_spool_usage(&mut self) {
    self.max_msg_spool_usage = None;
  }

  pub fn set_max_subscription_count(&mut self, max_subscription_count: i64) {
    self.max_subscription_count = Some(max_subscription_count);
  }

  pub fn with_max_subscription_count(mut self, max_subscription_count: i64) -> MsgVpn {
    self.max_subscription_count = Some(max_subscription_count);
    self
  }

  pub fn max_subscription_count(&self) -> Option<&i64> {
    self.max_subscription_count.as_ref()
  }

  pub fn reset_max_subscription_count(&mut self) {
    self.max_subscription_count = None;
  }

  pub fn set_max_transacted_session_count(&mut self, max_transacted_session_count: i64) {
    self.max_transacted_session_count = Some(max_transacted_session_count);
  }

  pub fn with_max_transacted_session_count(mut self, max_transacted_session_count: i64) -> MsgVpn {
    self.max_transacted_session_count = Some(max_transacted_session_count);
    self
  }

  pub fn max_transacted_session_count(&self) -> Option<&i64> {
    self.max_transacted_session_count.as_ref()
  }

  pub fn reset_max_transacted_session_count(&mut self) {
    self.max_transacted_session_count = None;
  }

  pub fn set_max_transaction_count(&mut self, max_transaction_count: i64) {
    self.max_transaction_count = Some(max_transaction_count);
  }

  pub fn with_max_transaction_count(mut self, max_transaction_count: i64) -> MsgVpn {
    self.max_transaction_count = Some(max_transaction_count);
    self
  }

  pub fn max_transaction_count(&self) -> Option<&i64> {
    self.max_transaction_count.as_ref()
  }

  pub fn reset_max_transaction_count(&mut self) {
    self.max_transaction_count = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpn {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_replication_ack_propagation_interval_msg_count(&mut self, replication_ack_propagation_interval_msg_count: i64) {
    self.replication_ack_propagation_interval_msg_count = Some(replication_ack_propagation_interval_msg_count);
  }

  pub fn with_replication_ack_propagation_interval_msg_count(mut self, replication_ack_propagation_interval_msg_count: i64) -> MsgVpn {
    self.replication_ack_propagation_interval_msg_count = Some(replication_ack_propagation_interval_msg_count);
    self
  }

  pub fn replication_ack_propagation_interval_msg_count(&self) -> Option<&i64> {
    self.replication_ack_propagation_interval_msg_count.as_ref()
  }

  pub fn reset_replication_ack_propagation_interval_msg_count(&mut self) {
    self.replication_ack_propagation_interval_msg_count = None;
  }

  pub fn set_replication_bridge_authentication_basic_client_username(&mut self, replication_bridge_authentication_basic_client_username: String) {
    self.replication_bridge_authentication_basic_client_username = Some(replication_bridge_authentication_basic_client_username);
  }

  pub fn with_replication_bridge_authentication_basic_client_username(mut self, replication_bridge_authentication_basic_client_username: String) -> MsgVpn {
    self.replication_bridge_authentication_basic_client_username = Some(replication_bridge_authentication_basic_client_username);
    self
  }

  pub fn replication_bridge_authentication_basic_client_username(&self) -> Option<&String> {
    self.replication_bridge_authentication_basic_client_username.as_ref()
  }

  pub fn reset_replication_bridge_authentication_basic_client_username(&mut self) {
    self.replication_bridge_authentication_basic_client_username = None;
  }

  pub fn set_replication_bridge_authentication_basic_password(&mut self, replication_bridge_authentication_basic_password: String) {
    self.replication_bridge_authentication_basic_password = Some(replication_bridge_authentication_basic_password);
  }

  pub fn with_replication_bridge_authentication_basic_password(mut self, replication_bridge_authentication_basic_password: String) -> MsgVpn {
    self.replication_bridge_authentication_basic_password = Some(replication_bridge_authentication_basic_password);
    self
  }

  pub fn replication_bridge_authentication_basic_password(&self) -> Option<&String> {
    self.replication_bridge_authentication_basic_password.as_ref()
  }

  pub fn reset_replication_bridge_authentication_basic_password(&mut self) {
    self.replication_bridge_authentication_basic_password = None;
  }

  pub fn set_replication_bridge_authentication_client_cert_content(&mut self, replication_bridge_authentication_client_cert_content: String) {
    self.replication_bridge_authentication_client_cert_content = Some(replication_bridge_authentication_client_cert_content);
  }

  pub fn with_replication_bridge_authentication_client_cert_content(mut self, replication_bridge_authentication_client_cert_content: String) -> MsgVpn {
    self.replication_bridge_authentication_client_cert_content = Some(replication_bridge_authentication_client_cert_content);
    self
  }

  pub fn replication_bridge_authentication_client_cert_content(&self) -> Option<&String> {
    self.replication_bridge_authentication_client_cert_content.as_ref()
  }

  pub fn reset_replication_bridge_authentication_client_cert_content(&mut self) {
    self.replication_bridge_authentication_client_cert_content = None;
  }

  pub fn set_replication_bridge_authentication_client_cert_password(&mut self, replication_bridge_authentication_client_cert_password: String) {
    self.replication_bridge_authentication_client_cert_password = Some(replication_bridge_authentication_client_cert_password);
  }

  pub fn with_replication_bridge_authentication_client_cert_password(mut self, replication_bridge_authentication_client_cert_password: String) -> MsgVpn {
    self.replication_bridge_authentication_client_cert_password = Some(replication_bridge_authentication_client_cert_password);
    self
  }

  pub fn replication_bridge_authentication_client_cert_password(&self) -> Option<&String> {
    self.replication_bridge_authentication_client_cert_password.as_ref()
  }

  pub fn reset_replication_bridge_authentication_client_cert_password(&mut self) {
    self.replication_bridge_authentication_client_cert_password = None;
  }

  pub fn set_replication_bridge_authentication_scheme(&mut self, replication_bridge_authentication_scheme: String) {
    self.replication_bridge_authentication_scheme = Some(replication_bridge_authentication_scheme);
  }

  pub fn with_replication_bridge_authentication_scheme(mut self, replication_bridge_authentication_scheme: String) -> MsgVpn {
    self.replication_bridge_authentication_scheme = Some(replication_bridge_authentication_scheme);
    self
  }

  pub fn replication_bridge_authentication_scheme(&self) -> Option<&String> {
    self.replication_bridge_authentication_scheme.as_ref()
  }

  pub fn reset_replication_bridge_authentication_scheme(&mut self) {
    self.replication_bridge_authentication_scheme = None;
  }

  pub fn set_replication_bridge_compressed_data_enabled(&mut self, replication_bridge_compressed_data_enabled: bool) {
    self.replication_bridge_compressed_data_enabled = Some(replication_bridge_compressed_data_enabled);
  }

  pub fn with_replication_bridge_compressed_data_enabled(mut self, replication_bridge_compressed_data_enabled: bool) -> MsgVpn {
    self.replication_bridge_compressed_data_enabled = Some(replication_bridge_compressed_data_enabled);
    self
  }

  pub fn replication_bridge_compressed_data_enabled(&self) -> Option<&bool> {
    self.replication_bridge_compressed_data_enabled.as_ref()
  }

  pub fn reset_replication_bridge_compressed_data_enabled(&mut self) {
    self.replication_bridge_compressed_data_enabled = None;
  }

  pub fn set_replication_bridge_egress_flow_window_size(&mut self, replication_bridge_egress_flow_window_size: i64) {
    self.replication_bridge_egress_flow_window_size = Some(replication_bridge_egress_flow_window_size);
  }

  pub fn with_replication_bridge_egress_flow_window_size(mut self, replication_bridge_egress_flow_window_size: i64) -> MsgVpn {
    self.replication_bridge_egress_flow_window_size = Some(replication_bridge_egress_flow_window_size);
    self
  }

  pub fn replication_bridge_egress_flow_window_size(&self) -> Option<&i64> {
    self.replication_bridge_egress_flow_window_size.as_ref()
  }

  pub fn reset_replication_bridge_egress_flow_window_size(&mut self) {
    self.replication_bridge_egress_flow_window_size = None;
  }

  pub fn set_replication_bridge_retry_delay(&mut self, replication_bridge_retry_delay: i64) {
    self.replication_bridge_retry_delay = Some(replication_bridge_retry_delay);
  }

  pub fn with_replication_bridge_retry_delay(mut self, replication_bridge_retry_delay: i64) -> MsgVpn {
    self.replication_bridge_retry_delay = Some(replication_bridge_retry_delay);
    self
  }

  pub fn replication_bridge_retry_delay(&self) -> Option<&i64> {
    self.replication_bridge_retry_delay.as_ref()
  }

  pub fn reset_replication_bridge_retry_delay(&mut self) {
    self.replication_bridge_retry_delay = None;
  }

  pub fn set_replication_bridge_tls_enabled(&mut self, replication_bridge_tls_enabled: bool) {
    self.replication_bridge_tls_enabled = Some(replication_bridge_tls_enabled);
  }

  pub fn with_replication_bridge_tls_enabled(mut self, replication_bridge_tls_enabled: bool) -> MsgVpn {
    self.replication_bridge_tls_enabled = Some(replication_bridge_tls_enabled);
    self
  }

  pub fn replication_bridge_tls_enabled(&self) -> Option<&bool> {
    self.replication_bridge_tls_enabled.as_ref()
  }

  pub fn reset_replication_bridge_tls_enabled(&mut self) {
    self.replication_bridge_tls_enabled = None;
  }

  pub fn set_replication_bridge_unidirectional_client_profile_name(&mut self, replication_bridge_unidirectional_client_profile_name: String) {
    self.replication_bridge_unidirectional_client_profile_name = Some(replication_bridge_unidirectional_client_profile_name);
  }

  pub fn with_replication_bridge_unidirectional_client_profile_name(mut self, replication_bridge_unidirectional_client_profile_name: String) -> MsgVpn {
    self.replication_bridge_unidirectional_client_profile_name = Some(replication_bridge_unidirectional_client_profile_name);
    self
  }

  pub fn replication_bridge_unidirectional_client_profile_name(&self) -> Option<&String> {
    self.replication_bridge_unidirectional_client_profile_name.as_ref()
  }

  pub fn reset_replication_bridge_unidirectional_client_profile_name(&mut self) {
    self.replication_bridge_unidirectional_client_profile_name = None;
  }

  pub fn set_replication_enabled(&mut self, replication_enabled: bool) {
    self.replication_enabled = Some(replication_enabled);
  }

  pub fn with_replication_enabled(mut self, replication_enabled: bool) -> MsgVpn {
    self.replication_enabled = Some(replication_enabled);
    self
  }

  pub fn replication_enabled(&self) -> Option<&bool> {
    self.replication_enabled.as_ref()
  }

  pub fn reset_replication_enabled(&mut self) {
    self.replication_enabled = None;
  }

  pub fn set_replication_enabled_queue_behavior(&mut self, replication_enabled_queue_behavior: String) {
    self.replication_enabled_queue_behavior = Some(replication_enabled_queue_behavior);
  }

  pub fn with_replication_enabled_queue_behavior(mut self, replication_enabled_queue_behavior: String) -> MsgVpn {
    self.replication_enabled_queue_behavior = Some(replication_enabled_queue_behavior);
    self
  }

  pub fn replication_enabled_queue_behavior(&self) -> Option<&String> {
    self.replication_enabled_queue_behavior.as_ref()
  }

  pub fn reset_replication_enabled_queue_behavior(&mut self) {
    self.replication_enabled_queue_behavior = None;
  }

  pub fn set_replication_queue_max_msg_spool_usage(&mut self, replication_queue_max_msg_spool_usage: i64) {
    self.replication_queue_max_msg_spool_usage = Some(replication_queue_max_msg_spool_usage);
  }

  pub fn with_replication_queue_max_msg_spool_usage(mut self, replication_queue_max_msg_spool_usage: i64) -> MsgVpn {
    self.replication_queue_max_msg_spool_usage = Some(replication_queue_max_msg_spool_usage);
    self
  }

  pub fn replication_queue_max_msg_spool_usage(&self) -> Option<&i64> {
    self.replication_queue_max_msg_spool_usage.as_ref()
  }

  pub fn reset_replication_queue_max_msg_spool_usage(&mut self) {
    self.replication_queue_max_msg_spool_usage = None;
  }

  pub fn set_replication_queue_reject_msg_to_sender_on_discard_enabled(&mut self, replication_queue_reject_msg_to_sender_on_discard_enabled: bool) {
    self.replication_queue_reject_msg_to_sender_on_discard_enabled = Some(replication_queue_reject_msg_to_sender_on_discard_enabled);
  }

  pub fn with_replication_queue_reject_msg_to_sender_on_discard_enabled(mut self, replication_queue_reject_msg_to_sender_on_discard_enabled: bool) -> MsgVpn {
    self.replication_queue_reject_msg_to_sender_on_discard_enabled = Some(replication_queue_reject_msg_to_sender_on_discard_enabled);
    self
  }

  pub fn replication_queue_reject_msg_to_sender_on_discard_enabled(&self) -> Option<&bool> {
    self.replication_queue_reject_msg_to_sender_on_discard_enabled.as_ref()
  }

  pub fn reset_replication_queue_reject_msg_to_sender_on_discard_enabled(&mut self) {
    self.replication_queue_reject_msg_to_sender_on_discard_enabled = None;
  }

  pub fn set_replication_reject_msg_when_sync_ineligible_enabled(&mut self, replication_reject_msg_when_sync_ineligible_enabled: bool) {
    self.replication_reject_msg_when_sync_ineligible_enabled = Some(replication_reject_msg_when_sync_ineligible_enabled);
  }

  pub fn with_replication_reject_msg_when_sync_ineligible_enabled(mut self, replication_reject_msg_when_sync_ineligible_enabled: bool) -> MsgVpn {
    self.replication_reject_msg_when_sync_ineligible_enabled = Some(replication_reject_msg_when_sync_ineligible_enabled);
    self
  }

  pub fn replication_reject_msg_when_sync_ineligible_enabled(&self) -> Option<&bool> {
    self.replication_reject_msg_when_sync_ineligible_enabled.as_ref()
  }

  pub fn reset_replication_reject_msg_when_sync_ineligible_enabled(&mut self) {
    self.replication_reject_msg_when_sync_ineligible_enabled = None;
  }

  pub fn set_replication_role(&mut self, replication_role: String) {
    self.replication_role = Some(replication_role);
  }

  pub fn with_replication_role(mut self, replication_role: String) -> MsgVpn {
    self.replication_role = Some(replication_role);
    self
  }

  pub fn replication_role(&self) -> Option<&String> {
    self.replication_role.as_ref()
  }

  pub fn reset_replication_role(&mut self) {
    self.replication_role = None;
  }

  pub fn set_replication_transaction_mode(&mut self, replication_transaction_mode: String) {
    self.replication_transaction_mode = Some(replication_transaction_mode);
  }

  pub fn with_replication_transaction_mode(mut self, replication_transaction_mode: String) -> MsgVpn {
    self.replication_transaction_mode = Some(replication_transaction_mode);
    self
  }

  pub fn replication_transaction_mode(&self) -> Option<&String> {
    self.replication_transaction_mode.as_ref()
  }

  pub fn reset_replication_transaction_mode(&mut self) {
    self.replication_transaction_mode = None;
  }

  pub fn set_rest_tls_server_cert_enforce_trusted_common_name_enabled(&mut self, rest_tls_server_cert_enforce_trusted_common_name_enabled: bool) {
    self.rest_tls_server_cert_enforce_trusted_common_name_enabled = Some(rest_tls_server_cert_enforce_trusted_common_name_enabled);
  }

  pub fn with_rest_tls_server_cert_enforce_trusted_common_name_enabled(mut self, rest_tls_server_cert_enforce_trusted_common_name_enabled: bool) -> MsgVpn {
    self.rest_tls_server_cert_enforce_trusted_common_name_enabled = Some(rest_tls_server_cert_enforce_trusted_common_name_enabled);
    self
  }

  pub fn rest_tls_server_cert_enforce_trusted_common_name_enabled(&self) -> Option<&bool> {
    self.rest_tls_server_cert_enforce_trusted_common_name_enabled.as_ref()
  }

  pub fn reset_rest_tls_server_cert_enforce_trusted_common_name_enabled(&mut self) {
    self.rest_tls_server_cert_enforce_trusted_common_name_enabled = None;
  }

  pub fn set_rest_tls_server_cert_max_chain_depth(&mut self, rest_tls_server_cert_max_chain_depth: i64) {
    self.rest_tls_server_cert_max_chain_depth = Some(rest_tls_server_cert_max_chain_depth);
  }

  pub fn with_rest_tls_server_cert_max_chain_depth(mut self, rest_tls_server_cert_max_chain_depth: i64) -> MsgVpn {
    self.rest_tls_server_cert_max_chain_depth = Some(rest_tls_server_cert_max_chain_depth);
    self
  }

  pub fn rest_tls_server_cert_max_chain_depth(&self) -> Option<&i64> {
    self.rest_tls_server_cert_max_chain_depth.as_ref()
  }

  pub fn reset_rest_tls_server_cert_max_chain_depth(&mut self) {
    self.rest_tls_server_cert_max_chain_depth = None;
  }

  pub fn set_rest_tls_server_cert_validate_date_enabled(&mut self, rest_tls_server_cert_validate_date_enabled: bool) {
    self.rest_tls_server_cert_validate_date_enabled = Some(rest_tls_server_cert_validate_date_enabled);
  }

  pub fn with_rest_tls_server_cert_validate_date_enabled(mut self, rest_tls_server_cert_validate_date_enabled: bool) -> MsgVpn {
    self.rest_tls_server_cert_validate_date_enabled = Some(rest_tls_server_cert_validate_date_enabled);
    self
  }

  pub fn rest_tls_server_cert_validate_date_enabled(&self) -> Option<&bool> {
    self.rest_tls_server_cert_validate_date_enabled.as_ref()
  }

  pub fn reset_rest_tls_server_cert_validate_date_enabled(&mut self) {
    self.rest_tls_server_cert_validate_date_enabled = None;
  }

  pub fn set_semp_over_msg_bus_admin_client_enabled(&mut self, semp_over_msg_bus_admin_client_enabled: bool) {
    self.semp_over_msg_bus_admin_client_enabled = Some(semp_over_msg_bus_admin_client_enabled);
  }

  pub fn with_semp_over_msg_bus_admin_client_enabled(mut self, semp_over_msg_bus_admin_client_enabled: bool) -> MsgVpn {
    self.semp_over_msg_bus_admin_client_enabled = Some(semp_over_msg_bus_admin_client_enabled);
    self
  }

  pub fn semp_over_msg_bus_admin_client_enabled(&self) -> Option<&bool> {
    self.semp_over_msg_bus_admin_client_enabled.as_ref()
  }

  pub fn reset_semp_over_msg_bus_admin_client_enabled(&mut self) {
    self.semp_over_msg_bus_admin_client_enabled = None;
  }

  pub fn set_semp_over_msg_bus_admin_distributed_cache_enabled(&mut self, semp_over_msg_bus_admin_distributed_cache_enabled: bool) {
    self.semp_over_msg_bus_admin_distributed_cache_enabled = Some(semp_over_msg_bus_admin_distributed_cache_enabled);
  }

  pub fn with_semp_over_msg_bus_admin_distributed_cache_enabled(mut self, semp_over_msg_bus_admin_distributed_cache_enabled: bool) -> MsgVpn {
    self.semp_over_msg_bus_admin_distributed_cache_enabled = Some(semp_over_msg_bus_admin_distributed_cache_enabled);
    self
  }

  pub fn semp_over_msg_bus_admin_distributed_cache_enabled(&self) -> Option<&bool> {
    self.semp_over_msg_bus_admin_distributed_cache_enabled.as_ref()
  }

  pub fn reset_semp_over_msg_bus_admin_distributed_cache_enabled(&mut self) {
    self.semp_over_msg_bus_admin_distributed_cache_enabled = None;
  }

  pub fn set_semp_over_msg_bus_admin_enabled(&mut self, semp_over_msg_bus_admin_enabled: bool) {
    self.semp_over_msg_bus_admin_enabled = Some(semp_over_msg_bus_admin_enabled);
  }

  pub fn with_semp_over_msg_bus_admin_enabled(mut self, semp_over_msg_bus_admin_enabled: bool) -> MsgVpn {
    self.semp_over_msg_bus_admin_enabled = Some(semp_over_msg_bus_admin_enabled);
    self
  }

  pub fn semp_over_msg_bus_admin_enabled(&self) -> Option<&bool> {
    self.semp_over_msg_bus_admin_enabled.as_ref()
  }

  pub fn reset_semp_over_msg_bus_admin_enabled(&mut self) {
    self.semp_over_msg_bus_admin_enabled = None;
  }

  pub fn set_semp_over_msg_bus_enabled(&mut self, semp_over_msg_bus_enabled: bool) {
    self.semp_over_msg_bus_enabled = Some(semp_over_msg_bus_enabled);
  }

  pub fn with_semp_over_msg_bus_enabled(mut self, semp_over_msg_bus_enabled: bool) -> MsgVpn {
    self.semp_over_msg_bus_enabled = Some(semp_over_msg_bus_enabled);
    self
  }

  pub fn semp_over_msg_bus_enabled(&self) -> Option<&bool> {
    self.semp_over_msg_bus_enabled.as_ref()
  }

  pub fn reset_semp_over_msg_bus_enabled(&mut self) {
    self.semp_over_msg_bus_enabled = None;
  }

  pub fn set_semp_over_msg_bus_show_enabled(&mut self, semp_over_msg_bus_show_enabled: bool) {
    self.semp_over_msg_bus_show_enabled = Some(semp_over_msg_bus_show_enabled);
  }

  pub fn with_semp_over_msg_bus_show_enabled(mut self, semp_over_msg_bus_show_enabled: bool) -> MsgVpn {
    self.semp_over_msg_bus_show_enabled = Some(semp_over_msg_bus_show_enabled);
    self
  }

  pub fn semp_over_msg_bus_show_enabled(&self) -> Option<&bool> {
    self.semp_over_msg_bus_show_enabled.as_ref()
  }

  pub fn reset_semp_over_msg_bus_show_enabled(&mut self) {
    self.semp_over_msg_bus_show_enabled = None;
  }

  pub fn set_service_amqp_max_connection_count(&mut self, service_amqp_max_connection_count: i64) {
    self.service_amqp_max_connection_count = Some(service_amqp_max_connection_count);
  }

  pub fn with_service_amqp_max_connection_count(mut self, service_amqp_max_connection_count: i64) -> MsgVpn {
    self.service_amqp_max_connection_count = Some(service_amqp_max_connection_count);
    self
  }

  pub fn service_amqp_max_connection_count(&self) -> Option<&i64> {
    self.service_amqp_max_connection_count.as_ref()
  }

  pub fn reset_service_amqp_max_connection_count(&mut self) {
    self.service_amqp_max_connection_count = None;
  }

  pub fn set_service_amqp_plain_text_enabled(&mut self, service_amqp_plain_text_enabled: bool) {
    self.service_amqp_plain_text_enabled = Some(service_amqp_plain_text_enabled);
  }

  pub fn with_service_amqp_plain_text_enabled(mut self, service_amqp_plain_text_enabled: bool) -> MsgVpn {
    self.service_amqp_plain_text_enabled = Some(service_amqp_plain_text_enabled);
    self
  }

  pub fn service_amqp_plain_text_enabled(&self) -> Option<&bool> {
    self.service_amqp_plain_text_enabled.as_ref()
  }

  pub fn reset_service_amqp_plain_text_enabled(&mut self) {
    self.service_amqp_plain_text_enabled = None;
  }

  pub fn set_service_amqp_plain_text_listen_port(&mut self, service_amqp_plain_text_listen_port: i64) {
    self.service_amqp_plain_text_listen_port = Some(service_amqp_plain_text_listen_port);
  }

  pub fn with_service_amqp_plain_text_listen_port(mut self, service_amqp_plain_text_listen_port: i64) -> MsgVpn {
    self.service_amqp_plain_text_listen_port = Some(service_amqp_plain_text_listen_port);
    self
  }

  pub fn service_amqp_plain_text_listen_port(&self) -> Option<&i64> {
    self.service_amqp_plain_text_listen_port.as_ref()
  }

  pub fn reset_service_amqp_plain_text_listen_port(&mut self) {
    self.service_amqp_plain_text_listen_port = None;
  }

  pub fn set_service_amqp_tls_enabled(&mut self, service_amqp_tls_enabled: bool) {
    self.service_amqp_tls_enabled = Some(service_amqp_tls_enabled);
  }

  pub fn with_service_amqp_tls_enabled(mut self, service_amqp_tls_enabled: bool) -> MsgVpn {
    self.service_amqp_tls_enabled = Some(service_amqp_tls_enabled);
    self
  }

  pub fn service_amqp_tls_enabled(&self) -> Option<&bool> {
    self.service_amqp_tls_enabled.as_ref()
  }

  pub fn reset_service_amqp_tls_enabled(&mut self) {
    self.service_amqp_tls_enabled = None;
  }

  pub fn set_service_amqp_tls_listen_port(&mut self, service_amqp_tls_listen_port: i64) {
    self.service_amqp_tls_listen_port = Some(service_amqp_tls_listen_port);
  }

  pub fn with_service_amqp_tls_listen_port(mut self, service_amqp_tls_listen_port: i64) -> MsgVpn {
    self.service_amqp_tls_listen_port = Some(service_amqp_tls_listen_port);
    self
  }

  pub fn service_amqp_tls_listen_port(&self) -> Option<&i64> {
    self.service_amqp_tls_listen_port.as_ref()
  }

  pub fn reset_service_amqp_tls_listen_port(&mut self) {
    self.service_amqp_tls_listen_port = None;
  }

  pub fn set_service_mqtt_max_connection_count(&mut self, service_mqtt_max_connection_count: i64) {
    self.service_mqtt_max_connection_count = Some(service_mqtt_max_connection_count);
  }

  pub fn with_service_mqtt_max_connection_count(mut self, service_mqtt_max_connection_count: i64) -> MsgVpn {
    self.service_mqtt_max_connection_count = Some(service_mqtt_max_connection_count);
    self
  }

  pub fn service_mqtt_max_connection_count(&self) -> Option<&i64> {
    self.service_mqtt_max_connection_count.as_ref()
  }

  pub fn reset_service_mqtt_max_connection_count(&mut self) {
    self.service_mqtt_max_connection_count = None;
  }

  pub fn set_service_mqtt_plain_text_enabled(&mut self, service_mqtt_plain_text_enabled: bool) {
    self.service_mqtt_plain_text_enabled = Some(service_mqtt_plain_text_enabled);
  }

  pub fn with_service_mqtt_plain_text_enabled(mut self, service_mqtt_plain_text_enabled: bool) -> MsgVpn {
    self.service_mqtt_plain_text_enabled = Some(service_mqtt_plain_text_enabled);
    self
  }

  pub fn service_mqtt_plain_text_enabled(&self) -> Option<&bool> {
    self.service_mqtt_plain_text_enabled.as_ref()
  }

  pub fn reset_service_mqtt_plain_text_enabled(&mut self) {
    self.service_mqtt_plain_text_enabled = None;
  }

  pub fn set_service_mqtt_plain_text_listen_port(&mut self, service_mqtt_plain_text_listen_port: i64) {
    self.service_mqtt_plain_text_listen_port = Some(service_mqtt_plain_text_listen_port);
  }

  pub fn with_service_mqtt_plain_text_listen_port(mut self, service_mqtt_plain_text_listen_port: i64) -> MsgVpn {
    self.service_mqtt_plain_text_listen_port = Some(service_mqtt_plain_text_listen_port);
    self
  }

  pub fn service_mqtt_plain_text_listen_port(&self) -> Option<&i64> {
    self.service_mqtt_plain_text_listen_port.as_ref()
  }

  pub fn reset_service_mqtt_plain_text_listen_port(&mut self) {
    self.service_mqtt_plain_text_listen_port = None;
  }

  pub fn set_service_mqtt_tls_enabled(&mut self, service_mqtt_tls_enabled: bool) {
    self.service_mqtt_tls_enabled = Some(service_mqtt_tls_enabled);
  }

  pub fn with_service_mqtt_tls_enabled(mut self, service_mqtt_tls_enabled: bool) -> MsgVpn {
    self.service_mqtt_tls_enabled = Some(service_mqtt_tls_enabled);
    self
  }

  pub fn service_mqtt_tls_enabled(&self) -> Option<&bool> {
    self.service_mqtt_tls_enabled.as_ref()
  }

  pub fn reset_service_mqtt_tls_enabled(&mut self) {
    self.service_mqtt_tls_enabled = None;
  }

  pub fn set_service_mqtt_tls_listen_port(&mut self, service_mqtt_tls_listen_port: i64) {
    self.service_mqtt_tls_listen_port = Some(service_mqtt_tls_listen_port);
  }

  pub fn with_service_mqtt_tls_listen_port(mut self, service_mqtt_tls_listen_port: i64) -> MsgVpn {
    self.service_mqtt_tls_listen_port = Some(service_mqtt_tls_listen_port);
    self
  }

  pub fn service_mqtt_tls_listen_port(&self) -> Option<&i64> {
    self.service_mqtt_tls_listen_port.as_ref()
  }

  pub fn reset_service_mqtt_tls_listen_port(&mut self) {
    self.service_mqtt_tls_listen_port = None;
  }

  pub fn set_service_mqtt_tls_web_socket_enabled(&mut self, service_mqtt_tls_web_socket_enabled: bool) {
    self.service_mqtt_tls_web_socket_enabled = Some(service_mqtt_tls_web_socket_enabled);
  }

  pub fn with_service_mqtt_tls_web_socket_enabled(mut self, service_mqtt_tls_web_socket_enabled: bool) -> MsgVpn {
    self.service_mqtt_tls_web_socket_enabled = Some(service_mqtt_tls_web_socket_enabled);
    self
  }

  pub fn service_mqtt_tls_web_socket_enabled(&self) -> Option<&bool> {
    self.service_mqtt_tls_web_socket_enabled.as_ref()
  }

  pub fn reset_service_mqtt_tls_web_socket_enabled(&mut self) {
    self.service_mqtt_tls_web_socket_enabled = None;
  }

  pub fn set_service_mqtt_tls_web_socket_listen_port(&mut self, service_mqtt_tls_web_socket_listen_port: i64) {
    self.service_mqtt_tls_web_socket_listen_port = Some(service_mqtt_tls_web_socket_listen_port);
  }

  pub fn with_service_mqtt_tls_web_socket_listen_port(mut self, service_mqtt_tls_web_socket_listen_port: i64) -> MsgVpn {
    self.service_mqtt_tls_web_socket_listen_port = Some(service_mqtt_tls_web_socket_listen_port);
    self
  }

  pub fn service_mqtt_tls_web_socket_listen_port(&self) -> Option<&i64> {
    self.service_mqtt_tls_web_socket_listen_port.as_ref()
  }

  pub fn reset_service_mqtt_tls_web_socket_listen_port(&mut self) {
    self.service_mqtt_tls_web_socket_listen_port = None;
  }

  pub fn set_service_mqtt_web_socket_enabled(&mut self, service_mqtt_web_socket_enabled: bool) {
    self.service_mqtt_web_socket_enabled = Some(service_mqtt_web_socket_enabled);
  }

  pub fn with_service_mqtt_web_socket_enabled(mut self, service_mqtt_web_socket_enabled: bool) -> MsgVpn {
    self.service_mqtt_web_socket_enabled = Some(service_mqtt_web_socket_enabled);
    self
  }

  pub fn service_mqtt_web_socket_enabled(&self) -> Option<&bool> {
    self.service_mqtt_web_socket_enabled.as_ref()
  }

  pub fn reset_service_mqtt_web_socket_enabled(&mut self) {
    self.service_mqtt_web_socket_enabled = None;
  }

  pub fn set_service_mqtt_web_socket_listen_port(&mut self, service_mqtt_web_socket_listen_port: i64) {
    self.service_mqtt_web_socket_listen_port = Some(service_mqtt_web_socket_listen_port);
  }

  pub fn with_service_mqtt_web_socket_listen_port(mut self, service_mqtt_web_socket_listen_port: i64) -> MsgVpn {
    self.service_mqtt_web_socket_listen_port = Some(service_mqtt_web_socket_listen_port);
    self
  }

  pub fn service_mqtt_web_socket_listen_port(&self) -> Option<&i64> {
    self.service_mqtt_web_socket_listen_port.as_ref()
  }

  pub fn reset_service_mqtt_web_socket_listen_port(&mut self) {
    self.service_mqtt_web_socket_listen_port = None;
  }

  pub fn set_service_rest_incoming_max_connection_count(&mut self, service_rest_incoming_max_connection_count: i64) {
    self.service_rest_incoming_max_connection_count = Some(service_rest_incoming_max_connection_count);
  }

  pub fn with_service_rest_incoming_max_connection_count(mut self, service_rest_incoming_max_connection_count: i64) -> MsgVpn {
    self.service_rest_incoming_max_connection_count = Some(service_rest_incoming_max_connection_count);
    self
  }

  pub fn service_rest_incoming_max_connection_count(&self) -> Option<&i64> {
    self.service_rest_incoming_max_connection_count.as_ref()
  }

  pub fn reset_service_rest_incoming_max_connection_count(&mut self) {
    self.service_rest_incoming_max_connection_count = None;
  }

  pub fn set_service_rest_incoming_plain_text_enabled(&mut self, service_rest_incoming_plain_text_enabled: bool) {
    self.service_rest_incoming_plain_text_enabled = Some(service_rest_incoming_plain_text_enabled);
  }

  pub fn with_service_rest_incoming_plain_text_enabled(mut self, service_rest_incoming_plain_text_enabled: bool) -> MsgVpn {
    self.service_rest_incoming_plain_text_enabled = Some(service_rest_incoming_plain_text_enabled);
    self
  }

  pub fn service_rest_incoming_plain_text_enabled(&self) -> Option<&bool> {
    self.service_rest_incoming_plain_text_enabled.as_ref()
  }

  pub fn reset_service_rest_incoming_plain_text_enabled(&mut self) {
    self.service_rest_incoming_plain_text_enabled = None;
  }

  pub fn set_service_rest_incoming_plain_text_listen_port(&mut self, service_rest_incoming_plain_text_listen_port: i64) {
    self.service_rest_incoming_plain_text_listen_port = Some(service_rest_incoming_plain_text_listen_port);
  }

  pub fn with_service_rest_incoming_plain_text_listen_port(mut self, service_rest_incoming_plain_text_listen_port: i64) -> MsgVpn {
    self.service_rest_incoming_plain_text_listen_port = Some(service_rest_incoming_plain_text_listen_port);
    self
  }

  pub fn service_rest_incoming_plain_text_listen_port(&self) -> Option<&i64> {
    self.service_rest_incoming_plain_text_listen_port.as_ref()
  }

  pub fn reset_service_rest_incoming_plain_text_listen_port(&mut self) {
    self.service_rest_incoming_plain_text_listen_port = None;
  }

  pub fn set_service_rest_incoming_tls_enabled(&mut self, service_rest_incoming_tls_enabled: bool) {
    self.service_rest_incoming_tls_enabled = Some(service_rest_incoming_tls_enabled);
  }

  pub fn with_service_rest_incoming_tls_enabled(mut self, service_rest_incoming_tls_enabled: bool) -> MsgVpn {
    self.service_rest_incoming_tls_enabled = Some(service_rest_incoming_tls_enabled);
    self
  }

  pub fn service_rest_incoming_tls_enabled(&self) -> Option<&bool> {
    self.service_rest_incoming_tls_enabled.as_ref()
  }

  pub fn reset_service_rest_incoming_tls_enabled(&mut self) {
    self.service_rest_incoming_tls_enabled = None;
  }

  pub fn set_service_rest_incoming_tls_listen_port(&mut self, service_rest_incoming_tls_listen_port: i64) {
    self.service_rest_incoming_tls_listen_port = Some(service_rest_incoming_tls_listen_port);
  }

  pub fn with_service_rest_incoming_tls_listen_port(mut self, service_rest_incoming_tls_listen_port: i64) -> MsgVpn {
    self.service_rest_incoming_tls_listen_port = Some(service_rest_incoming_tls_listen_port);
    self
  }

  pub fn service_rest_incoming_tls_listen_port(&self) -> Option<&i64> {
    self.service_rest_incoming_tls_listen_port.as_ref()
  }

  pub fn reset_service_rest_incoming_tls_listen_port(&mut self) {
    self.service_rest_incoming_tls_listen_port = None;
  }

  pub fn set_service_rest_mode(&mut self, service_rest_mode: String) {
    self.service_rest_mode = Some(service_rest_mode);
  }

  pub fn with_service_rest_mode(mut self, service_rest_mode: String) -> MsgVpn {
    self.service_rest_mode = Some(service_rest_mode);
    self
  }

  pub fn service_rest_mode(&self) -> Option<&String> {
    self.service_rest_mode.as_ref()
  }

  pub fn reset_service_rest_mode(&mut self) {
    self.service_rest_mode = None;
  }

  pub fn set_service_rest_outgoing_max_connection_count(&mut self, service_rest_outgoing_max_connection_count: i64) {
    self.service_rest_outgoing_max_connection_count = Some(service_rest_outgoing_max_connection_count);
  }

  pub fn with_service_rest_outgoing_max_connection_count(mut self, service_rest_outgoing_max_connection_count: i64) -> MsgVpn {
    self.service_rest_outgoing_max_connection_count = Some(service_rest_outgoing_max_connection_count);
    self
  }

  pub fn service_rest_outgoing_max_connection_count(&self) -> Option<&i64> {
    self.service_rest_outgoing_max_connection_count.as_ref()
  }

  pub fn reset_service_rest_outgoing_max_connection_count(&mut self) {
    self.service_rest_outgoing_max_connection_count = None;
  }

  pub fn set_service_smf_max_connection_count(&mut self, service_smf_max_connection_count: i64) {
    self.service_smf_max_connection_count = Some(service_smf_max_connection_count);
  }

  pub fn with_service_smf_max_connection_count(mut self, service_smf_max_connection_count: i64) -> MsgVpn {
    self.service_smf_max_connection_count = Some(service_smf_max_connection_count);
    self
  }

  pub fn service_smf_max_connection_count(&self) -> Option<&i64> {
    self.service_smf_max_connection_count.as_ref()
  }

  pub fn reset_service_smf_max_connection_count(&mut self) {
    self.service_smf_max_connection_count = None;
  }

  pub fn set_service_smf_plain_text_enabled(&mut self, service_smf_plain_text_enabled: bool) {
    self.service_smf_plain_text_enabled = Some(service_smf_plain_text_enabled);
  }

  pub fn with_service_smf_plain_text_enabled(mut self, service_smf_plain_text_enabled: bool) -> MsgVpn {
    self.service_smf_plain_text_enabled = Some(service_smf_plain_text_enabled);
    self
  }

  pub fn service_smf_plain_text_enabled(&self) -> Option<&bool> {
    self.service_smf_plain_text_enabled.as_ref()
  }

  pub fn reset_service_smf_plain_text_enabled(&mut self) {
    self.service_smf_plain_text_enabled = None;
  }

  pub fn set_service_smf_tls_enabled(&mut self, service_smf_tls_enabled: bool) {
    self.service_smf_tls_enabled = Some(service_smf_tls_enabled);
  }

  pub fn with_service_smf_tls_enabled(mut self, service_smf_tls_enabled: bool) -> MsgVpn {
    self.service_smf_tls_enabled = Some(service_smf_tls_enabled);
    self
  }

  pub fn service_smf_tls_enabled(&self) -> Option<&bool> {
    self.service_smf_tls_enabled.as_ref()
  }

  pub fn reset_service_smf_tls_enabled(&mut self) {
    self.service_smf_tls_enabled = None;
  }

  pub fn set_service_web_max_connection_count(&mut self, service_web_max_connection_count: i64) {
    self.service_web_max_connection_count = Some(service_web_max_connection_count);
  }

  pub fn with_service_web_max_connection_count(mut self, service_web_max_connection_count: i64) -> MsgVpn {
    self.service_web_max_connection_count = Some(service_web_max_connection_count);
    self
  }

  pub fn service_web_max_connection_count(&self) -> Option<&i64> {
    self.service_web_max_connection_count.as_ref()
  }

  pub fn reset_service_web_max_connection_count(&mut self) {
    self.service_web_max_connection_count = None;
  }

  pub fn set_service_web_plain_text_enabled(&mut self, service_web_plain_text_enabled: bool) {
    self.service_web_plain_text_enabled = Some(service_web_plain_text_enabled);
  }

  pub fn with_service_web_plain_text_enabled(mut self, service_web_plain_text_enabled: bool) -> MsgVpn {
    self.service_web_plain_text_enabled = Some(service_web_plain_text_enabled);
    self
  }

  pub fn service_web_plain_text_enabled(&self) -> Option<&bool> {
    self.service_web_plain_text_enabled.as_ref()
  }

  pub fn reset_service_web_plain_text_enabled(&mut self) {
    self.service_web_plain_text_enabled = None;
  }

  pub fn set_service_web_tls_enabled(&mut self, service_web_tls_enabled: bool) {
    self.service_web_tls_enabled = Some(service_web_tls_enabled);
  }

  pub fn with_service_web_tls_enabled(mut self, service_web_tls_enabled: bool) -> MsgVpn {
    self.service_web_tls_enabled = Some(service_web_tls_enabled);
    self
  }

  pub fn service_web_tls_enabled(&self) -> Option<&bool> {
    self.service_web_tls_enabled.as_ref()
  }

  pub fn reset_service_web_tls_enabled(&mut self) {
    self.service_web_tls_enabled = None;
  }

  pub fn set_tls_allow_downgrade_to_plain_text_enabled(&mut self, tls_allow_downgrade_to_plain_text_enabled: bool) {
    self.tls_allow_downgrade_to_plain_text_enabled = Some(tls_allow_downgrade_to_plain_text_enabled);
  }

  pub fn with_tls_allow_downgrade_to_plain_text_enabled(mut self, tls_allow_downgrade_to_plain_text_enabled: bool) -> MsgVpn {
    self.tls_allow_downgrade_to_plain_text_enabled = Some(tls_allow_downgrade_to_plain_text_enabled);
    self
  }

  pub fn tls_allow_downgrade_to_plain_text_enabled(&self) -> Option<&bool> {
    self.tls_allow_downgrade_to_plain_text_enabled.as_ref()
  }

  pub fn reset_tls_allow_downgrade_to_plain_text_enabled(&mut self) {
    self.tls_allow_downgrade_to_plain_text_enabled = None;
  }

}



