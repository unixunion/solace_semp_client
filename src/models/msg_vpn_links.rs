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
pub struct MsgVpnLinks {
  /// The URI of this MsgVpn's aclProfiles collection.
  #[serde(rename = "aclProfilesUri")]
  acl_profiles_uri: Option<String>,
  /// The URI of this MsgVpn's authorizationGroups collection.
  #[serde(rename = "authorizationGroupsUri")]
  authorization_groups_uri: Option<String>,
  /// The URI of this MsgVpn's bridges collection.
  #[serde(rename = "bridgesUri")]
  bridges_uri: Option<String>,
  /// The URI of this MsgVpn's clientProfiles collection.
  #[serde(rename = "clientProfilesUri")]
  client_profiles_uri: Option<String>,
  /// The URI of this MsgVpn's clientUsernames collection.
  #[serde(rename = "clientUsernamesUri")]
  client_usernames_uri: Option<String>,
  /// The URI of this MsgVpn's jndiConnectionFactories collection. Available since 2.2.
  #[serde(rename = "jndiConnectionFactoriesUri")]
  jndi_connection_factories_uri: Option<String>,
  /// The URI of this MsgVpn's jndiQueues collection. Available since 2.2.
  #[serde(rename = "jndiQueuesUri")]
  jndi_queues_uri: Option<String>,
  /// The URI of this MsgVpn's jndiTopics collection. Available since 2.2.
  #[serde(rename = "jndiTopicsUri")]
  jndi_topics_uri: Option<String>,
  /// The URI of this MsgVpn's mqttSessions collection. Available since 2.1.
  #[serde(rename = "mqttSessionsUri")]
  mqtt_sessions_uri: Option<String>,
  /// The URI of this MsgVpn's queues collection.
  #[serde(rename = "queuesUri")]
  queues_uri: Option<String>,
  /// The URI of this MsgVpn's replayLogs collection. Available since 2.10.
  #[serde(rename = "replayLogsUri")]
  replay_logs_uri: Option<String>,
  /// The URI of this MsgVpn's replicatedTopics collection. Available since 2.9.
  #[serde(rename = "replicatedTopicsUri")]
  replicated_topics_uri: Option<String>,
  /// The URI of this MsgVpn's restDeliveryPoints collection.
  #[serde(rename = "restDeliveryPointsUri")]
  rest_delivery_points_uri: Option<String>,
  /// The URI of this MsgVpn's sequencedTopics collection.
  #[serde(rename = "sequencedTopicsUri")]
  sequenced_topics_uri: Option<String>,
  /// The URI of this MsgVpn's topicEndpoints collection. Available since 2.1.
  #[serde(rename = "topicEndpointsUri")]
  topic_endpoints_uri: Option<String>,
  /// The URI of this MsgVpn object.
  #[serde(rename = "uri")]
  uri: Option<String>
}

impl MsgVpnLinks {
  pub fn new() -> MsgVpnLinks {
    MsgVpnLinks {
      acl_profiles_uri: None,
      authorization_groups_uri: None,
      bridges_uri: None,
      client_profiles_uri: None,
      client_usernames_uri: None,
      jndi_connection_factories_uri: None,
      jndi_queues_uri: None,
      jndi_topics_uri: None,
      mqtt_sessions_uri: None,
      queues_uri: None,
      replay_logs_uri: None,
      replicated_topics_uri: None,
      rest_delivery_points_uri: None,
      sequenced_topics_uri: None,
      topic_endpoints_uri: None,
      uri: None
    }
  }

  pub fn set_acl_profiles_uri(&mut self, acl_profiles_uri: String) {
    self.acl_profiles_uri = Some(acl_profiles_uri);
  }

  pub fn with_acl_profiles_uri(mut self, acl_profiles_uri: String) -> MsgVpnLinks {
    self.acl_profiles_uri = Some(acl_profiles_uri);
    self
  }

  pub fn acl_profiles_uri(&self) -> Option<&String> {
    self.acl_profiles_uri.as_ref()
  }

  pub fn reset_acl_profiles_uri(&mut self) {
    self.acl_profiles_uri = None;
  }

  pub fn set_authorization_groups_uri(&mut self, authorization_groups_uri: String) {
    self.authorization_groups_uri = Some(authorization_groups_uri);
  }

  pub fn with_authorization_groups_uri(mut self, authorization_groups_uri: String) -> MsgVpnLinks {
    self.authorization_groups_uri = Some(authorization_groups_uri);
    self
  }

  pub fn authorization_groups_uri(&self) -> Option<&String> {
    self.authorization_groups_uri.as_ref()
  }

  pub fn reset_authorization_groups_uri(&mut self) {
    self.authorization_groups_uri = None;
  }

  pub fn set_bridges_uri(&mut self, bridges_uri: String) {
    self.bridges_uri = Some(bridges_uri);
  }

  pub fn with_bridges_uri(mut self, bridges_uri: String) -> MsgVpnLinks {
    self.bridges_uri = Some(bridges_uri);
    self
  }

  pub fn bridges_uri(&self) -> Option<&String> {
    self.bridges_uri.as_ref()
  }

  pub fn reset_bridges_uri(&mut self) {
    self.bridges_uri = None;
  }

  pub fn set_client_profiles_uri(&mut self, client_profiles_uri: String) {
    self.client_profiles_uri = Some(client_profiles_uri);
  }

  pub fn with_client_profiles_uri(mut self, client_profiles_uri: String) -> MsgVpnLinks {
    self.client_profiles_uri = Some(client_profiles_uri);
    self
  }

  pub fn client_profiles_uri(&self) -> Option<&String> {
    self.client_profiles_uri.as_ref()
  }

  pub fn reset_client_profiles_uri(&mut self) {
    self.client_profiles_uri = None;
  }

  pub fn set_client_usernames_uri(&mut self, client_usernames_uri: String) {
    self.client_usernames_uri = Some(client_usernames_uri);
  }

  pub fn with_client_usernames_uri(mut self, client_usernames_uri: String) -> MsgVpnLinks {
    self.client_usernames_uri = Some(client_usernames_uri);
    self
  }

  pub fn client_usernames_uri(&self) -> Option<&String> {
    self.client_usernames_uri.as_ref()
  }

  pub fn reset_client_usernames_uri(&mut self) {
    self.client_usernames_uri = None;
  }

  pub fn set_jndi_connection_factories_uri(&mut self, jndi_connection_factories_uri: String) {
    self.jndi_connection_factories_uri = Some(jndi_connection_factories_uri);
  }

  pub fn with_jndi_connection_factories_uri(mut self, jndi_connection_factories_uri: String) -> MsgVpnLinks {
    self.jndi_connection_factories_uri = Some(jndi_connection_factories_uri);
    self
  }

  pub fn jndi_connection_factories_uri(&self) -> Option<&String> {
    self.jndi_connection_factories_uri.as_ref()
  }

  pub fn reset_jndi_connection_factories_uri(&mut self) {
    self.jndi_connection_factories_uri = None;
  }

  pub fn set_jndi_queues_uri(&mut self, jndi_queues_uri: String) {
    self.jndi_queues_uri = Some(jndi_queues_uri);
  }

  pub fn with_jndi_queues_uri(mut self, jndi_queues_uri: String) -> MsgVpnLinks {
    self.jndi_queues_uri = Some(jndi_queues_uri);
    self
  }

  pub fn jndi_queues_uri(&self) -> Option<&String> {
    self.jndi_queues_uri.as_ref()
  }

  pub fn reset_jndi_queues_uri(&mut self) {
    self.jndi_queues_uri = None;
  }

  pub fn set_jndi_topics_uri(&mut self, jndi_topics_uri: String) {
    self.jndi_topics_uri = Some(jndi_topics_uri);
  }

  pub fn with_jndi_topics_uri(mut self, jndi_topics_uri: String) -> MsgVpnLinks {
    self.jndi_topics_uri = Some(jndi_topics_uri);
    self
  }

  pub fn jndi_topics_uri(&self) -> Option<&String> {
    self.jndi_topics_uri.as_ref()
  }

  pub fn reset_jndi_topics_uri(&mut self) {
    self.jndi_topics_uri = None;
  }

  pub fn set_mqtt_sessions_uri(&mut self, mqtt_sessions_uri: String) {
    self.mqtt_sessions_uri = Some(mqtt_sessions_uri);
  }

  pub fn with_mqtt_sessions_uri(mut self, mqtt_sessions_uri: String) -> MsgVpnLinks {
    self.mqtt_sessions_uri = Some(mqtt_sessions_uri);
    self
  }

  pub fn mqtt_sessions_uri(&self) -> Option<&String> {
    self.mqtt_sessions_uri.as_ref()
  }

  pub fn reset_mqtt_sessions_uri(&mut self) {
    self.mqtt_sessions_uri = None;
  }

  pub fn set_queues_uri(&mut self, queues_uri: String) {
    self.queues_uri = Some(queues_uri);
  }

  pub fn with_queues_uri(mut self, queues_uri: String) -> MsgVpnLinks {
    self.queues_uri = Some(queues_uri);
    self
  }

  pub fn queues_uri(&self) -> Option<&String> {
    self.queues_uri.as_ref()
  }

  pub fn reset_queues_uri(&mut self) {
    self.queues_uri = None;
  }

  pub fn set_replay_logs_uri(&mut self, replay_logs_uri: String) {
    self.replay_logs_uri = Some(replay_logs_uri);
  }

  pub fn with_replay_logs_uri(mut self, replay_logs_uri: String) -> MsgVpnLinks {
    self.replay_logs_uri = Some(replay_logs_uri);
    self
  }

  pub fn replay_logs_uri(&self) -> Option<&String> {
    self.replay_logs_uri.as_ref()
  }

  pub fn reset_replay_logs_uri(&mut self) {
    self.replay_logs_uri = None;
  }

  pub fn set_replicated_topics_uri(&mut self, replicated_topics_uri: String) {
    self.replicated_topics_uri = Some(replicated_topics_uri);
  }

  pub fn with_replicated_topics_uri(mut self, replicated_topics_uri: String) -> MsgVpnLinks {
    self.replicated_topics_uri = Some(replicated_topics_uri);
    self
  }

  pub fn replicated_topics_uri(&self) -> Option<&String> {
    self.replicated_topics_uri.as_ref()
  }

  pub fn reset_replicated_topics_uri(&mut self) {
    self.replicated_topics_uri = None;
  }

  pub fn set_rest_delivery_points_uri(&mut self, rest_delivery_points_uri: String) {
    self.rest_delivery_points_uri = Some(rest_delivery_points_uri);
  }

  pub fn with_rest_delivery_points_uri(mut self, rest_delivery_points_uri: String) -> MsgVpnLinks {
    self.rest_delivery_points_uri = Some(rest_delivery_points_uri);
    self
  }

  pub fn rest_delivery_points_uri(&self) -> Option<&String> {
    self.rest_delivery_points_uri.as_ref()
  }

  pub fn reset_rest_delivery_points_uri(&mut self) {
    self.rest_delivery_points_uri = None;
  }

  pub fn set_sequenced_topics_uri(&mut self, sequenced_topics_uri: String) {
    self.sequenced_topics_uri = Some(sequenced_topics_uri);
  }

  pub fn with_sequenced_topics_uri(mut self, sequenced_topics_uri: String) -> MsgVpnLinks {
    self.sequenced_topics_uri = Some(sequenced_topics_uri);
    self
  }

  pub fn sequenced_topics_uri(&self) -> Option<&String> {
    self.sequenced_topics_uri.as_ref()
  }

  pub fn reset_sequenced_topics_uri(&mut self) {
    self.sequenced_topics_uri = None;
  }

  pub fn set_topic_endpoints_uri(&mut self, topic_endpoints_uri: String) {
    self.topic_endpoints_uri = Some(topic_endpoints_uri);
  }

  pub fn with_topic_endpoints_uri(mut self, topic_endpoints_uri: String) -> MsgVpnLinks {
    self.topic_endpoints_uri = Some(topic_endpoints_uri);
    self
  }

  pub fn topic_endpoints_uri(&self) -> Option<&String> {
    self.topic_endpoints_uri.as_ref()
  }

  pub fn reset_topic_endpoints_uri(&mut self) {
    self.topic_endpoints_uri = None;
  }

  pub fn set_uri(&mut self, uri: String) {
    self.uri = Some(uri);
  }

  pub fn with_uri(mut self, uri: String) -> MsgVpnLinks {
    self.uri = Some(uri);
    self
  }

  pub fn uri(&self) -> Option<&String> {
    self.uri.as_ref()
  }

  pub fn reset_uri(&mut self) {
    self.uri = None;
  }

}



