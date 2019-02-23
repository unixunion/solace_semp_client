/* 
 * SEMP (Solace Element Management Protocol)
 *
 * SEMP (starting in `v2`, see [note 1](#notes)) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+  broker. Resources are either individual **objects**, or **collections** of  objects. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Configuration|/SEMP/v2/config|Reading and writing config state|See [note 2](#notes)    Resources are always nouns, with individual objects being singular and  collections being plural. Objects within a collection are identified by an  `obj-id`, which follows the collection name with the form  `collection-name/obj-id`. Some examples:  <pre> /SEMP/v2/config/msgVpns                       ; MsgVpn collection /SEMP/v2/config/msgVpns/finance               ; MsgVpn object named \"finance\" /SEMP/v2/config/msgVpns/finance/queues        ; Queue collection within MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance/queues/orderQ ; Queue object named \"orderQ\" within MsgVpn \"finance\" </pre>  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and  are described by JSON arrays. Each item in the array represents an object in  the same manner as the individual object would normally be represented. The creation of a new object is done through its collection  resource.   ## Object Resources  Objects are composed of attributes and collections, and are described by JSON  content as name/value pairs. The collections of an object are not contained  directly in the object's JSON content, rather the content includes a URI  attribute which points to the collection. This contained collection resource  must be managed as a separate resource through this URI.  At a minimum, every object has 1 or more identifying attributes, and its own  `uri` attribute which contains the URI to itself. Attributes may have any  (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See [note 3](#notes) Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in  certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request     ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these  general principles:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see [note 4](#notes)) PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many  method/URI combinations. Individual URIs may document additional parameters.  Note that multiple query parameters can be used together in a single URI,  separated by the ampersand character. For example:  <pre> ; Request for the MsgVpns collection using two hypothetical query parameters ; \"q1\" and \"q2\" with values \"val1\" and \"val2\" respectively /SEMP/v2/config/msgVpns?q1=val1&q2=val2 </pre>  ### select  Include in the response only selected attributes of the object, or exclude  from the response selected attributes of the object. Use this query parameter  to limit the size of the returned data for each returned object, return only  those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the  list contains attribute names that are not prefaced by `-`, only those  attributes are included in the response. If the list contains attribute names  that are prefaced by `-`, those attributes are excluded from the response. If  the list contains both types, then the difference of the first set of  attributes and the second set of attributes is returned. If the list is  empty (i.e. `select=`), no attributes are returned  All attributes that are prefaced by `-` must follow all attributes that are  not prefaced by `-`. In addition, each attribute name in the list must match  at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute  names are supported using periods (e.g. `parentName.childName`).  Some examples:  <pre> ; List of all MsgVpn names /SEMP/v2/config/msgVpns?select=msgVpnName  ; List of all MsgVpn and their attributes except for their names /SEMP/v2/config/msgVpns?select=-msgVpnName  ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance?select=authentication*  ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/config/msgVpns/finance?select=-authentication*  ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance/queues/orderQ?select=owner,permission </pre>  ### where  Include in the response only objects where certain conditions are true. Use  this query parameter to limit which objects are returned to those whose  attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions  must be true for the object to be included in the response. Each expression  takes the form:  <pre> expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' </pre>  `value` may be a number, string, `true`, or `false`, as appropriate for the  type of `attribute-name`. Greater-than and less-than comparisons only work for  numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more  characters). Some examples:  <pre> ; Only enabled MsgVpns /SEMP/v2/config/msgVpns?where=enabled==true  ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/config/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap  ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/config/msgVpns?where=maxConnectionCount>100  ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/config/msgVpns?where=msgVpnName==B* </pre>  ### count  Limit the count of objects in the response. This can be useful to limit the  size of the response for large collections. The minimum value for `count` is  `1` and the default is `10`. There is a hidden maximum  as to prevent overloading the system. For example:  <pre> ; Up to 25 MsgVpns /SEMP/v2/config/msgVpns?count=25 </pre>  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data  that should not be created or interpreted by SEMP clients, and should only be  used as described below.  When a request is made for a collection and there may be additional objects  available for retrieval that are not included in the initial response, the  response will include a `cursorQuery` field containing a cursor. The value  of this field can be specified in the `cursor` query parameter of a  subsequent request to retrieve the next page of objects. For convenience,  an appropriate URI is constructed automatically by the broker and included  in the `nextPageUri` field of the response. This URI can be used directly  to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first. 5|For DELETE, the body of the request currently serves no purpose and will cause an error if not empty.    
 *
 * OpenAPI spec version: 2.10
 * Contact: support@solace.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use hyper::header::UserAgent;

use super::{Error, configuration};

pub struct DefaultApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> DefaultApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> DefaultApiClient<C> {
        DefaultApiClient {
            configuration: configuration,
        }
    }
}

pub trait DefaultApi {
    fn create_msg_vpn(&self, body: ::models::MsgVpn, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_acl_profile(&self, msg_vpn_name: &str, body: ::models::MsgVpnAclProfile, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_acl_profile_client_connect_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, body: ::models::MsgVpnAclProfileClientConnectException, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileClientConnectExceptionResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_acl_profile_publish_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, body: ::models::MsgVpnAclProfilePublishException, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfilePublishExceptionResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_acl_profile_subscribe_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, body: ::models::MsgVpnAclProfileSubscribeException, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileSubscribeExceptionResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_authorization_group(&self, msg_vpn_name: &str, body: ::models::MsgVpnAuthorizationGroup, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAuthorizationGroupResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_bridge(&self, msg_vpn_name: &str, body: ::models::MsgVpnBridge, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_bridge_remote_msg_vpn(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, body: ::models::MsgVpnBridgeRemoteMsgVpn, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteMsgVpnResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_bridge_remote_subscription(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, body: ::models::MsgVpnBridgeRemoteSubscription, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteSubscriptionResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_bridge_tls_trusted_common_name(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, body: ::models::MsgVpnBridgeTlsTrustedCommonName, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeTlsTrustedCommonNameResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_client_profile(&self, msg_vpn_name: &str, body: ::models::MsgVpnClientProfile, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientProfileResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_client_username(&self, msg_vpn_name: &str, body: ::models::MsgVpnClientUsername, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientUsernameResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_jndi_connection_factory(&self, msg_vpn_name: &str, body: ::models::MsgVpnJndiConnectionFactory, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiConnectionFactoryResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_jndi_queue(&self, msg_vpn_name: &str, body: ::models::MsgVpnJndiQueue, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiQueueResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_jndi_topic(&self, msg_vpn_name: &str, body: ::models::MsgVpnJndiTopic, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiTopicResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_mqtt_session(&self, msg_vpn_name: &str, body: ::models::MsgVpnMqttSession, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_mqtt_session_subscription(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, body: ::models::MsgVpnMqttSessionSubscription, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionSubscriptionResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_queue(&self, msg_vpn_name: &str, body: ::models::MsgVpnQueue, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_queue_subscription(&self, msg_vpn_name: &str, queue_name: &str, body: ::models::MsgVpnQueueSubscription, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueSubscriptionResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_replay_log(&self, msg_vpn_name: &str, body: ::models::MsgVpnReplayLog, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplayLogResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_replicated_topic(&self, msg_vpn_name: &str, body: ::models::MsgVpnReplicatedTopic, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplicatedTopicResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_rest_delivery_point(&self, msg_vpn_name: &str, body: ::models::MsgVpnRestDeliveryPoint, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_rest_delivery_point_queue_binding(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, body: ::models::MsgVpnRestDeliveryPointQueueBinding, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointQueueBindingResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_rest_delivery_point_rest_consumer(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, body: ::models::MsgVpnRestDeliveryPointRestConsumer, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, body: ::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonName, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_sequenced_topic(&self, msg_vpn_name: &str, body: ::models::MsgVpnSequencedTopic, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnSequencedTopicResponse, Error = Error<serde_json::Value>>>;
    fn create_msg_vpn_topic_endpoint(&self, msg_vpn_name: &str, body: ::models::MsgVpnTopicEndpoint, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn(&self, msg_vpn_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_acl_profile(&self, msg_vpn_name: &str, acl_profile_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_acl_profile_client_connect_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, client_connect_exception_address: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_acl_profile_publish_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, topic_syntax: &str, publish_exception_topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_acl_profile_subscribe_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, topic_syntax: &str, subscribe_exception_topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_authorization_group(&self, msg_vpn_name: &str, authorization_group_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_bridge(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_bridge_remote_msg_vpn(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_msg_vpn_name: &str, remote_msg_vpn_location: &str, remote_msg_vpn_interface: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_bridge_remote_subscription(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_subscription_topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_bridge_tls_trusted_common_name(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, tls_trusted_common_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_client_profile(&self, msg_vpn_name: &str, client_profile_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_client_username(&self, msg_vpn_name: &str, client_username: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_jndi_connection_factory(&self, msg_vpn_name: &str, connection_factory_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_jndi_queue(&self, msg_vpn_name: &str, queue_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_jndi_topic(&self, msg_vpn_name: &str, topic_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_mqtt_session(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_mqtt_session_subscription(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, subscription_topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_queue(&self, msg_vpn_name: &str, queue_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_queue_subscription(&self, msg_vpn_name: &str, queue_name: &str, subscription_topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_replay_log(&self, msg_vpn_name: &str, replay_log_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_replicated_topic(&self, msg_vpn_name: &str, replicated_topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_rest_delivery_point(&self, msg_vpn_name: &str, rest_delivery_point_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_rest_delivery_point_queue_binding(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, queue_binding_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_rest_delivery_point_rest_consumer(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, tls_trusted_common_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_sequenced_topic(&self, msg_vpn_name: &str, sequenced_topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn delete_msg_vpn_topic_endpoint(&self, msg_vpn_name: &str, topic_endpoint_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>>;
    fn get_about_user_msg_vpn(&self, msg_vpn_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::AboutUserMsgVpnResponse, Error = Error<serde_json::Value>>>;
    fn get_about_user_msg_vpns(&self, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::AboutUserMsgVpnsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn(&self, msg_vpn_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_acl_profile(&self, msg_vpn_name: &str, acl_profile_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_acl_profile_client_connect_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, client_connect_exception_address: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileClientConnectExceptionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_acl_profile_client_connect_exceptions(&self, msg_vpn_name: &str, acl_profile_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileClientConnectExceptionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_acl_profile_publish_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, topic_syntax: &str, publish_exception_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfilePublishExceptionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_acl_profile_publish_exceptions(&self, msg_vpn_name: &str, acl_profile_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfilePublishExceptionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_acl_profile_subscribe_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, topic_syntax: &str, subscribe_exception_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileSubscribeExceptionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_acl_profile_subscribe_exceptions(&self, msg_vpn_name: &str, acl_profile_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileSubscribeExceptionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_authorization_group(&self, msg_vpn_name: &str, authorization_group_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAuthorizationGroupResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge_remote_msg_vpn(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_msg_vpn_name: &str, remote_msg_vpn_location: &str, remote_msg_vpn_interface: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteMsgVpnResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge_remote_msg_vpns(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteMsgVpnsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge_remote_subscription(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteSubscriptionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge_remote_subscriptions(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteSubscriptionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge_tls_trusted_common_name(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, tls_trusted_common_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeTlsTrustedCommonNameResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_bridge_tls_trusted_common_names(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeTlsTrustedCommonNamesResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_client_profile(&self, msg_vpn_name: &str, client_profile_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientProfileResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_client_username(&self, msg_vpn_name: &str, client_username: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientUsernameResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_jndi_connection_factory(&self, msg_vpn_name: &str, connection_factory_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiConnectionFactoryResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_jndi_queue(&self, msg_vpn_name: &str, queue_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiQueueResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_jndi_topic(&self, msg_vpn_name: &str, topic_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiTopicResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_mqtt_session(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_mqtt_session_subscription(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionSubscriptionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_mqtt_session_subscriptions(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionSubscriptionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_queue(&self, msg_vpn_name: &str, queue_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_queue_subscription(&self, msg_vpn_name: &str, queue_name: &str, subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueSubscriptionResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_queue_subscriptions(&self, msg_vpn_name: &str, queue_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueSubscriptionsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_replay_log(&self, msg_vpn_name: &str, replay_log_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplayLogResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_replicated_topic(&self, msg_vpn_name: &str, replicated_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplicatedTopicResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_rest_delivery_point(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_rest_delivery_point_queue_binding(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, queue_binding_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointQueueBindingResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_rest_delivery_point_queue_bindings(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointQueueBindingsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_rest_delivery_point_rest_consumer(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, tls_trusted_common_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNamesResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_rest_delivery_point_rest_consumers(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumersResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_sequenced_topic(&self, msg_vpn_name: &str, sequenced_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnSequencedTopicResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_sequenced_topics(&self, msg_vpn_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnSequencedTopicsResponse, Error = Error<serde_json::Value>>>;
    fn get_msg_vpn_topic_endpoint(&self, msg_vpn_name: &str, topic_endpoint_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn(&self, msg_vpn_name: &str, body: ::models::MsgVpn, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_acl_profile(&self, msg_vpn_name: &str, acl_profile_name: &str, body: ::models::MsgVpnAclProfile, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_authorization_group(&self, msg_vpn_name: &str, authorization_group_name: &str, body: ::models::MsgVpnAuthorizationGroup, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAuthorizationGroupResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_bridge(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, body: ::models::MsgVpnBridge, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_bridge_remote_msg_vpn(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_msg_vpn_name: &str, remote_msg_vpn_location: &str, remote_msg_vpn_interface: &str, body: ::models::MsgVpnBridgeRemoteMsgVpn, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteMsgVpnResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_client_profile(&self, msg_vpn_name: &str, client_profile_name: &str, body: ::models::MsgVpnClientProfile, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientProfileResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_client_username(&self, msg_vpn_name: &str, client_username: &str, body: ::models::MsgVpnClientUsername, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientUsernameResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_jndi_connection_factory(&self, msg_vpn_name: &str, connection_factory_name: &str, body: ::models::MsgVpnJndiConnectionFactory, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiConnectionFactoryResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_jndi_queue(&self, msg_vpn_name: &str, queue_name: &str, body: ::models::MsgVpnJndiQueue, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiQueueResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_jndi_topic(&self, msg_vpn_name: &str, topic_name: &str, body: ::models::MsgVpnJndiTopic, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiTopicResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_mqtt_session(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, body: ::models::MsgVpnMqttSession, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_mqtt_session_subscription(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, subscription_topic: &str, body: ::models::MsgVpnMqttSessionSubscription, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionSubscriptionResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_queue(&self, msg_vpn_name: &str, queue_name: &str, body: ::models::MsgVpnQueue, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_replay_log(&self, msg_vpn_name: &str, replay_log_name: &str, body: ::models::MsgVpnReplayLog, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplayLogResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_replicated_topic(&self, msg_vpn_name: &str, replicated_topic: &str, body: ::models::MsgVpnReplicatedTopic, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplicatedTopicResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_rest_delivery_point(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, body: ::models::MsgVpnRestDeliveryPoint, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_rest_delivery_point_queue_binding(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, queue_binding_name: &str, body: ::models::MsgVpnRestDeliveryPointQueueBinding, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointQueueBindingResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_rest_delivery_point_rest_consumer(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, body: ::models::MsgVpnRestDeliveryPointRestConsumer, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerResponse, Error = Error<serde_json::Value>>>;
    fn replace_msg_vpn_topic_endpoint(&self, msg_vpn_name: &str, topic_endpoint_name: &str, body: ::models::MsgVpnTopicEndpoint, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn(&self, msg_vpn_name: &str, body: ::models::MsgVpn, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_acl_profile(&self, msg_vpn_name: &str, acl_profile_name: &str, body: ::models::MsgVpnAclProfile, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_authorization_group(&self, msg_vpn_name: &str, authorization_group_name: &str, body: ::models::MsgVpnAuthorizationGroup, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAuthorizationGroupResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_bridge(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, body: ::models::MsgVpnBridge, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_bridge_remote_msg_vpn(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_msg_vpn_name: &str, remote_msg_vpn_location: &str, remote_msg_vpn_interface: &str, body: ::models::MsgVpnBridgeRemoteMsgVpn, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteMsgVpnResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_client_profile(&self, msg_vpn_name: &str, client_profile_name: &str, body: ::models::MsgVpnClientProfile, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientProfileResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_client_username(&self, msg_vpn_name: &str, client_username: &str, body: ::models::MsgVpnClientUsername, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientUsernameResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_jndi_connection_factory(&self, msg_vpn_name: &str, connection_factory_name: &str, body: ::models::MsgVpnJndiConnectionFactory, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiConnectionFactoryResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_jndi_queue(&self, msg_vpn_name: &str, queue_name: &str, body: ::models::MsgVpnJndiQueue, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiQueueResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_jndi_topic(&self, msg_vpn_name: &str, topic_name: &str, body: ::models::MsgVpnJndiTopic, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiTopicResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_mqtt_session(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, body: ::models::MsgVpnMqttSession, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_mqtt_session_subscription(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, subscription_topic: &str, body: ::models::MsgVpnMqttSessionSubscription, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionSubscriptionResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_queue(&self, msg_vpn_name: &str, queue_name: &str, body: ::models::MsgVpnQueue, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_replay_log(&self, msg_vpn_name: &str, replay_log_name: &str, body: ::models::MsgVpnReplayLog, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplayLogResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_replicated_topic(&self, msg_vpn_name: &str, replicated_topic: &str, body: ::models::MsgVpnReplicatedTopic, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplicatedTopicResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_rest_delivery_point(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, body: ::models::MsgVpnRestDeliveryPoint, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_rest_delivery_point_queue_binding(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, queue_binding_name: &str, body: ::models::MsgVpnRestDeliveryPointQueueBinding, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointQueueBindingResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_rest_delivery_point_rest_consumer(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, body: ::models::MsgVpnRestDeliveryPointRestConsumer, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerResponse, Error = Error<serde_json::Value>>>;
    fn update_msg_vpn_topic_endpoint(&self, msg_vpn_name: &str, topic_endpoint_name: &str, body: ::models::MsgVpnTopicEndpoint, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointResponse, Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>DefaultApi for DefaultApiClient<C> {
    fn create_msg_vpn(&self, body: ::models::MsgVpn, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_acl_profile(&self, msg_vpn_name: &str, body: ::models::MsgVpnAclProfile, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfileResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_acl_profile_client_connect_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, body: ::models::MsgVpnAclProfileClientConnectException, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileClientConnectExceptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfileClientConnectExceptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_acl_profile_publish_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, body: ::models::MsgVpnAclProfilePublishException, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfilePublishExceptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfilePublishExceptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_acl_profile_subscribe_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, body: ::models::MsgVpnAclProfileSubscribeException, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileSubscribeExceptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfileSubscribeExceptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_authorization_group(&self, msg_vpn_name: &str, body: ::models::MsgVpnAuthorizationGroup, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAuthorizationGroupResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/authorizationGroups?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAuthorizationGroupResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_bridge(&self, msg_vpn_name: &str, body: ::models::MsgVpnBridge, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_bridge_remote_msg_vpn(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, body: ::models::MsgVpnBridgeRemoteMsgVpn, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteMsgVpnResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeRemoteMsgVpnResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_bridge_remote_subscription(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, body: ::models::MsgVpnBridgeRemoteSubscription, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteSubscriptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeRemoteSubscriptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_bridge_tls_trusted_common_name(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, body: ::models::MsgVpnBridgeTlsTrustedCommonName, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeTlsTrustedCommonNameResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeTlsTrustedCommonNameResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_client_profile(&self, msg_vpn_name: &str, body: ::models::MsgVpnClientProfile, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientProfileResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clientProfiles?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientProfileResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_client_username(&self, msg_vpn_name: &str, body: ::models::MsgVpnClientUsername, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientUsernameResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clientUsernames?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientUsernameResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_jndi_connection_factory(&self, msg_vpn_name: &str, body: ::models::MsgVpnJndiConnectionFactory, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiConnectionFactoryResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiConnectionFactories?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnJndiConnectionFactoryResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_jndi_queue(&self, msg_vpn_name: &str, body: ::models::MsgVpnJndiQueue, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiQueueResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiQueues?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnJndiQueueResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_jndi_topic(&self, msg_vpn_name: &str, body: ::models::MsgVpnJndiTopic, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiTopics?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnJndiTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_mqtt_session(&self, msg_vpn_name: &str, body: ::models::MsgVpnMqttSession, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/mqttSessions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnMqttSessionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_mqtt_session_subscription(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, body: ::models::MsgVpnMqttSessionSubscription, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionSubscriptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, mqttSessionClientId=mqtt_session_client_id, mqttSessionVirtualRouter=mqtt_session_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnMqttSessionSubscriptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_queue(&self, msg_vpn_name: &str, body: ::models::MsgVpnQueue, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueueResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_queue_subscription(&self, msg_vpn_name: &str, queue_name: &str, body: ::models::MsgVpnQueueSubscription, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueSubscriptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}/subscriptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueueSubscriptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_replay_log(&self, msg_vpn_name: &str, body: ::models::MsgVpnReplayLog, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplayLogResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/replayLogs?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnReplayLogResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_replicated_topic(&self, msg_vpn_name: &str, body: ::models::MsgVpnReplicatedTopic, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplicatedTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/replicatedTopics?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnReplicatedTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_rest_delivery_point(&self, msg_vpn_name: &str, body: ::models::MsgVpnRestDeliveryPoint, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_rest_delivery_point_queue_binding(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, body: ::models::MsgVpnRestDeliveryPointQueueBinding, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointQueueBindingResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointQueueBindingResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_rest_delivery_point_rest_consumer(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, body: ::models::MsgVpnRestDeliveryPointRestConsumer, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointRestConsumerResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, body: ::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonName, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, restConsumerName=rest_consumer_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_sequenced_topic(&self, msg_vpn_name: &str, body: ::models::MsgVpnSequencedTopic, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnSequencedTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/sequencedTopics?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnSequencedTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn create_msg_vpn_topic_endpoint(&self, msg_vpn_name: &str, body: ::models::MsgVpnTopicEndpoint, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/topicEndpoints?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTopicEndpointResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn(&self, msg_vpn_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_acl_profile(&self, msg_vpn_name: &str, acl_profile_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_acl_profile_client_connect_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, client_connect_exception_address: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions/{clientConnectExceptionAddress}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name, clientConnectExceptionAddress=client_connect_exception_address);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_acl_profile_publish_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, topic_syntax: &str, publish_exception_topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions/{topicSyntax},{publishExceptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name, topicSyntax=topic_syntax, publishExceptionTopic=publish_exception_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_acl_profile_subscribe_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, topic_syntax: &str, subscribe_exception_topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions/{topicSyntax},{subscribeExceptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name, topicSyntax=topic_syntax, subscribeExceptionTopic=subscribe_exception_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_authorization_group(&self, msg_vpn_name: &str, authorization_group_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, authorizationGroupName=authorization_group_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_bridge(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_bridge_remote_msg_vpn(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_msg_vpn_name: &str, remote_msg_vpn_location: &str, remote_msg_vpn_interface: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router, remoteMsgVpnName=remote_msg_vpn_name, remoteMsgVpnLocation=remote_msg_vpn_location, remoteMsgVpnInterface=remote_msg_vpn_interface);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_bridge_remote_subscription(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_subscription_topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions/{remoteSubscriptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router, remoteSubscriptionTopic=remote_subscription_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_bridge_tls_trusted_common_name(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, tls_trusted_common_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames/{tlsTrustedCommonName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router, tlsTrustedCommonName=tls_trusted_common_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_client_profile(&self, msg_vpn_name: &str, client_profile_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clientProfiles/{clientProfileName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientProfileName=client_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_client_username(&self, msg_vpn_name: &str, client_username: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clientUsernames/{clientUsername}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientUsername=client_username);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_jndi_connection_factory(&self, msg_vpn_name: &str, connection_factory_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, connectionFactoryName=connection_factory_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_jndi_queue(&self, msg_vpn_name: &str, queue_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiQueues/{queueName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_jndi_topic(&self, msg_vpn_name: &str, topic_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiTopics/{topicName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicName=topic_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_mqtt_session(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, mqttSessionClientId=mqtt_session_client_id, mqttSessionVirtualRouter=mqtt_session_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_mqtt_session_subscription(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, subscription_topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, mqttSessionClientId=mqtt_session_client_id, mqttSessionVirtualRouter=mqtt_session_virtual_router, subscriptionTopic=subscription_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_queue(&self, msg_vpn_name: &str, queue_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_queue_subscription(&self, msg_vpn_name: &str, queue_name: &str, subscription_topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}/subscriptions/{subscriptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name, subscriptionTopic=subscription_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_replay_log(&self, msg_vpn_name: &str, replay_log_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/replayLogs/{replayLogName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, replayLogName=replay_log_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_replicated_topic(&self, msg_vpn_name: &str, replicated_topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, replicatedTopic=replicated_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_rest_delivery_point(&self, msg_vpn_name: &str, rest_delivery_point_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_rest_delivery_point_queue_binding(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, queue_binding_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, queueBindingName=queue_binding_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_rest_delivery_point_rest_consumer(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, restConsumerName=rest_consumer_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, tls_trusted_common_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames/{tlsTrustedCommonName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, restConsumerName=rest_consumer_name, tlsTrustedCommonName=tls_trusted_common_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_sequenced_topic(&self, msg_vpn_name: &str, sequenced_topic: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/sequencedTopics/{sequencedTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, sequencedTopic=sequenced_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn delete_msg_vpn_topic_endpoint(&self, msg_vpn_name: &str, topic_endpoint_name: &str) -> Box<Future<Item = ::models::SempMetaOnlyResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Delete;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicEndpointName=topic_endpoint_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::SempMetaOnlyResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_about_user_msg_vpn(&self, msg_vpn_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::AboutUserMsgVpnResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/about/user/msgVpns/{msgVpnName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::AboutUserMsgVpnResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_about_user_msg_vpns(&self, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::AboutUserMsgVpnsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("count", &count.to_string());
            query.append_pair("cursor", &cursor.to_string());
            query.append_pair("where", &_where.join(",").to_string());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/about/user/msgVpns?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::AboutUserMsgVpnsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn(&self, msg_vpn_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_acl_profile(&self, msg_vpn_name: &str, acl_profile_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfileResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_acl_profile_client_connect_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, client_connect_exception_address: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileClientConnectExceptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions/{clientConnectExceptionAddress}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name, clientConnectExceptionAddress=client_connect_exception_address);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfileClientConnectExceptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_acl_profile_client_connect_exceptions(&self, msg_vpn_name: &str, acl_profile_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileClientConnectExceptionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("count", &count.to_string());
            query.append_pair("cursor", &cursor.to_string());
            query.append_pair("where", &_where.join(",").to_string());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfileClientConnectExceptionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_acl_profile_publish_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, topic_syntax: &str, publish_exception_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfilePublishExceptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions/{topicSyntax},{publishExceptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name, topicSyntax=topic_syntax, publishExceptionTopic=publish_exception_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfilePublishExceptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_acl_profile_publish_exceptions(&self, msg_vpn_name: &str, acl_profile_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfilePublishExceptionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("count", &count.to_string());
            query.append_pair("cursor", &cursor.to_string());
            query.append_pair("where", &_where.join(",").to_string());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfilePublishExceptionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_acl_profile_subscribe_exception(&self, msg_vpn_name: &str, acl_profile_name: &str, topic_syntax: &str, subscribe_exception_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileSubscribeExceptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions/{topicSyntax},{subscribeExceptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name, topicSyntax=topic_syntax, subscribeExceptionTopic=subscribe_exception_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfileSubscribeExceptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_acl_profile_subscribe_exceptions(&self, msg_vpn_name: &str, acl_profile_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileSubscribeExceptionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("count", &count.to_string());
            query.append_pair("cursor", &cursor.to_string());
            query.append_pair("where", &_where.join(",").to_string());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfileSubscribeExceptionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_authorization_group(&self, msg_vpn_name: &str, authorization_group_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAuthorizationGroupResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, authorizationGroupName=authorization_group_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAuthorizationGroupResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge_remote_msg_vpn(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_msg_vpn_name: &str, remote_msg_vpn_location: &str, remote_msg_vpn_interface: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteMsgVpnResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router, remoteMsgVpnName=remote_msg_vpn_name, remoteMsgVpnLocation=remote_msg_vpn_location, remoteMsgVpnInterface=remote_msg_vpn_interface);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeRemoteMsgVpnResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge_remote_msg_vpns(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteMsgVpnsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("where", &_where.join(",").to_string());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeRemoteMsgVpnsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge_remote_subscription(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteSubscriptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions/{remoteSubscriptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router, remoteSubscriptionTopic=remote_subscription_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeRemoteSubscriptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge_remote_subscriptions(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteSubscriptionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("count", &count.to_string());
            query.append_pair("cursor", &cursor.to_string());
            query.append_pair("where", &_where.join(",").to_string());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeRemoteSubscriptionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge_tls_trusted_common_name(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, tls_trusted_common_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeTlsTrustedCommonNameResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames/{tlsTrustedCommonName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router, tlsTrustedCommonName=tls_trusted_common_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeTlsTrustedCommonNameResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_bridge_tls_trusted_common_names(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeTlsTrustedCommonNamesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("where", &_where.join(",").to_string());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeTlsTrustedCommonNamesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_client_profile(&self, msg_vpn_name: &str, client_profile_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientProfileResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clientProfiles/{clientProfileName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientProfileName=client_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientProfileResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_client_username(&self, msg_vpn_name: &str, client_username: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientUsernameResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clientUsernames/{clientUsername}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientUsername=client_username);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientUsernameResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_jndi_connection_factory(&self, msg_vpn_name: &str, connection_factory_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiConnectionFactoryResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, connectionFactoryName=connection_factory_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnJndiConnectionFactoryResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_jndi_queue(&self, msg_vpn_name: &str, queue_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiQueueResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiQueues/{queueName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnJndiQueueResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_jndi_topic(&self, msg_vpn_name: &str, topic_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiTopics/{topicName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicName=topic_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnJndiTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_mqtt_session(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, mqttSessionClientId=mqtt_session_client_id, mqttSessionVirtualRouter=mqtt_session_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnMqttSessionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_mqtt_session_subscription(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionSubscriptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, mqttSessionClientId=mqtt_session_client_id, mqttSessionVirtualRouter=mqtt_session_virtual_router, subscriptionTopic=subscription_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnMqttSessionSubscriptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_mqtt_session_subscriptions(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionSubscriptionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("count", &count.to_string());
            query.append_pair("cursor", &cursor.to_string());
            query.append_pair("where", &_where.join(",").to_string());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, mqttSessionClientId=mqtt_session_client_id, mqttSessionVirtualRouter=mqtt_session_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnMqttSessionSubscriptionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_queue(&self, msg_vpn_name: &str, queue_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueueResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_queue_subscription(&self, msg_vpn_name: &str, queue_name: &str, subscription_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueSubscriptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}/subscriptions/{subscriptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name, subscriptionTopic=subscription_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueueSubscriptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_queue_subscriptions(&self, msg_vpn_name: &str, queue_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueSubscriptionsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("count", &count.to_string());
            query.append_pair("cursor", &cursor.to_string());
            query.append_pair("where", &_where.join(",").to_string());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}/subscriptions?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueueSubscriptionsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_replay_log(&self, msg_vpn_name: &str, replay_log_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplayLogResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/replayLogs/{replayLogName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, replayLogName=replay_log_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnReplayLogResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_replicated_topic(&self, msg_vpn_name: &str, replicated_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplicatedTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, replicatedTopic=replicated_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnReplicatedTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_rest_delivery_point(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_rest_delivery_point_queue_binding(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, queue_binding_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointQueueBindingResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, queueBindingName=queue_binding_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointQueueBindingResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_rest_delivery_point_queue_bindings(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointQueueBindingsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("count", &count.to_string());
            query.append_pair("cursor", &cursor.to_string());
            query.append_pair("where", &_where.join(",").to_string());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointQueueBindingsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_rest_delivery_point_rest_consumer(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, restConsumerName=rest_consumer_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointRestConsumerResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, tls_trusted_common_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames/{tlsTrustedCommonName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, restConsumerName=rest_consumer_name, tlsTrustedCommonName=tls_trusted_common_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNamesResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("where", &_where.join(",").to_string());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, restConsumerName=rest_consumer_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNamesResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_rest_delivery_point_rest_consumers(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumersResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("count", &count.to_string());
            query.append_pair("cursor", &cursor.to_string());
            query.append_pair("where", &_where.join(",").to_string());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointRestConsumersResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_sequenced_topic(&self, msg_vpn_name: &str, sequenced_topic: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnSequencedTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/sequencedTopics/{sequencedTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, sequencedTopic=sequenced_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnSequencedTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_sequenced_topics(&self, msg_vpn_name: &str, count: i32, cursor: &str, _where: Vec<String>, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnSequencedTopicsResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("count", &count.to_string());
            query.append_pair("cursor", &cursor.to_string());
            query.append_pair("where", &_where.join(",").to_string());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/sequencedTopics?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnSequencedTopicsResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_msg_vpn_topic_endpoint(&self, msg_vpn_name: &str, topic_endpoint_name: &str, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicEndpointName=topic_endpoint_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTopicEndpointResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn(&self, msg_vpn_name: &str, body: ::models::MsgVpn, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_acl_profile(&self, msg_vpn_name: &str, acl_profile_name: &str, body: ::models::MsgVpnAclProfile, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfileResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_authorization_group(&self, msg_vpn_name: &str, authorization_group_name: &str, body: ::models::MsgVpnAuthorizationGroup, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAuthorizationGroupResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, authorizationGroupName=authorization_group_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAuthorizationGroupResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_bridge(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, body: ::models::MsgVpnBridge, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_bridge_remote_msg_vpn(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_msg_vpn_name: &str, remote_msg_vpn_location: &str, remote_msg_vpn_interface: &str, body: ::models::MsgVpnBridgeRemoteMsgVpn, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteMsgVpnResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router, remoteMsgVpnName=remote_msg_vpn_name, remoteMsgVpnLocation=remote_msg_vpn_location, remoteMsgVpnInterface=remote_msg_vpn_interface);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeRemoteMsgVpnResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_client_profile(&self, msg_vpn_name: &str, client_profile_name: &str, body: ::models::MsgVpnClientProfile, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientProfileResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clientProfiles/{clientProfileName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientProfileName=client_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientProfileResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_client_username(&self, msg_vpn_name: &str, client_username: &str, body: ::models::MsgVpnClientUsername, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientUsernameResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clientUsernames/{clientUsername}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientUsername=client_username);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientUsernameResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_jndi_connection_factory(&self, msg_vpn_name: &str, connection_factory_name: &str, body: ::models::MsgVpnJndiConnectionFactory, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiConnectionFactoryResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, connectionFactoryName=connection_factory_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnJndiConnectionFactoryResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_jndi_queue(&self, msg_vpn_name: &str, queue_name: &str, body: ::models::MsgVpnJndiQueue, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiQueueResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiQueues/{queueName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnJndiQueueResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_jndi_topic(&self, msg_vpn_name: &str, topic_name: &str, body: ::models::MsgVpnJndiTopic, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiTopics/{topicName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicName=topic_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnJndiTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_mqtt_session(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, body: ::models::MsgVpnMqttSession, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, mqttSessionClientId=mqtt_session_client_id, mqttSessionVirtualRouter=mqtt_session_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnMqttSessionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_mqtt_session_subscription(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, subscription_topic: &str, body: ::models::MsgVpnMqttSessionSubscription, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionSubscriptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, mqttSessionClientId=mqtt_session_client_id, mqttSessionVirtualRouter=mqtt_session_virtual_router, subscriptionTopic=subscription_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnMqttSessionSubscriptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_queue(&self, msg_vpn_name: &str, queue_name: &str, body: ::models::MsgVpnQueue, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueueResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_replay_log(&self, msg_vpn_name: &str, replay_log_name: &str, body: ::models::MsgVpnReplayLog, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplayLogResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/replayLogs/{replayLogName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, replayLogName=replay_log_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnReplayLogResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_replicated_topic(&self, msg_vpn_name: &str, replicated_topic: &str, body: ::models::MsgVpnReplicatedTopic, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplicatedTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, replicatedTopic=replicated_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnReplicatedTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_rest_delivery_point(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, body: ::models::MsgVpnRestDeliveryPoint, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_rest_delivery_point_queue_binding(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, queue_binding_name: &str, body: ::models::MsgVpnRestDeliveryPointQueueBinding, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointQueueBindingResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, queueBindingName=queue_binding_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointQueueBindingResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_rest_delivery_point_rest_consumer(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, body: ::models::MsgVpnRestDeliveryPointRestConsumer, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, restConsumerName=rest_consumer_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointRestConsumerResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn replace_msg_vpn_topic_endpoint(&self, msg_vpn_name: &str, topic_endpoint_name: &str, body: ::models::MsgVpnTopicEndpoint, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicEndpointName=topic_endpoint_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTopicEndpointResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn(&self, msg_vpn_name: &str, body: ::models::MsgVpn, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_acl_profile(&self, msg_vpn_name: &str, acl_profile_name: &str, body: ::models::MsgVpnAclProfile, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAclProfileResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, aclProfileName=acl_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAclProfileResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_authorization_group(&self, msg_vpn_name: &str, authorization_group_name: &str, body: ::models::MsgVpnAuthorizationGroup, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnAuthorizationGroupResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, authorizationGroupName=authorization_group_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnAuthorizationGroupResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_bridge(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, body: ::models::MsgVpnBridge, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_bridge_remote_msg_vpn(&self, msg_vpn_name: &str, bridge_name: &str, bridge_virtual_router: &str, remote_msg_vpn_name: &str, remote_msg_vpn_location: &str, remote_msg_vpn_interface: &str, body: ::models::MsgVpnBridgeRemoteMsgVpn, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnBridgeRemoteMsgVpnResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, bridgeName=bridge_name, bridgeVirtualRouter=bridge_virtual_router, remoteMsgVpnName=remote_msg_vpn_name, remoteMsgVpnLocation=remote_msg_vpn_location, remoteMsgVpnInterface=remote_msg_vpn_interface);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnBridgeRemoteMsgVpnResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_client_profile(&self, msg_vpn_name: &str, client_profile_name: &str, body: ::models::MsgVpnClientProfile, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientProfileResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clientProfiles/{clientProfileName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientProfileName=client_profile_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientProfileResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_client_username(&self, msg_vpn_name: &str, client_username: &str, body: ::models::MsgVpnClientUsername, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnClientUsernameResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/clientUsernames/{clientUsername}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, clientUsername=client_username);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnClientUsernameResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_jndi_connection_factory(&self, msg_vpn_name: &str, connection_factory_name: &str, body: ::models::MsgVpnJndiConnectionFactory, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiConnectionFactoryResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, connectionFactoryName=connection_factory_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnJndiConnectionFactoryResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_jndi_queue(&self, msg_vpn_name: &str, queue_name: &str, body: ::models::MsgVpnJndiQueue, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiQueueResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiQueues/{queueName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnJndiQueueResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_jndi_topic(&self, msg_vpn_name: &str, topic_name: &str, body: ::models::MsgVpnJndiTopic, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnJndiTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/jndiTopics/{topicName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicName=topic_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnJndiTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_mqtt_session(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, body: ::models::MsgVpnMqttSession, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, mqttSessionClientId=mqtt_session_client_id, mqttSessionVirtualRouter=mqtt_session_virtual_router);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnMqttSessionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_mqtt_session_subscription(&self, msg_vpn_name: &str, mqtt_session_client_id: &str, mqtt_session_virtual_router: &str, subscription_topic: &str, body: ::models::MsgVpnMqttSessionSubscription, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnMqttSessionSubscriptionResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, mqttSessionClientId=mqtt_session_client_id, mqttSessionVirtualRouter=mqtt_session_virtual_router, subscriptionTopic=subscription_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnMqttSessionSubscriptionResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_queue(&self, msg_vpn_name: &str, queue_name: &str, body: ::models::MsgVpnQueue, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnQueueResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/queues/{queueName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, queueName=queue_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnQueueResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_replay_log(&self, msg_vpn_name: &str, replay_log_name: &str, body: ::models::MsgVpnReplayLog, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplayLogResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/replayLogs/{replayLogName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, replayLogName=replay_log_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnReplayLogResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_replicated_topic(&self, msg_vpn_name: &str, replicated_topic: &str, body: ::models::MsgVpnReplicatedTopic, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnReplicatedTopicResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, replicatedTopic=replicated_topic);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnReplicatedTopicResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_rest_delivery_point(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, body: ::models::MsgVpnRestDeliveryPoint, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_rest_delivery_point_queue_binding(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, queue_binding_name: &str, body: ::models::MsgVpnRestDeliveryPointQueueBinding, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointQueueBindingResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, queueBindingName=queue_binding_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointQueueBindingResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_rest_delivery_point_rest_consumer(&self, msg_vpn_name: &str, rest_delivery_point_name: &str, rest_consumer_name: &str, body: ::models::MsgVpnRestDeliveryPointRestConsumer, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnRestDeliveryPointRestConsumerResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, restDeliveryPointName=rest_delivery_point_name, restConsumerName=rest_consumer_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnRestDeliveryPointRestConsumerResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn update_msg_vpn_topic_endpoint(&self, msg_vpn_name: &str, topic_endpoint_name: &str, body: ::models::MsgVpnTopicEndpoint, select: Vec<String>) -> Box<Future<Item = ::models::MsgVpnTopicEndpointResponse, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let mut auth_query = HashMap::<String, String>::new();
        if let Some(ref auth_conf) = configuration.basic_auth {
            let auth = hyper::header::Authorization(
                hyper::header::Basic {
                    username: auth_conf.0.to_owned(),
                    password: auth_conf.1.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Patch;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("select", &select.join(",").to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName}?{}", configuration.base_path, query_string, msgVpnName=msg_vpn_name, topicEndpointName=topic_endpoint_name);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::MsgVpnTopicEndpointResponse, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

}
