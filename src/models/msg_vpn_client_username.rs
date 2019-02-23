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
pub struct MsgVpnClientUsername {
  /// The ACL Profile of the Client Username. The default value is `\"default\"`.
  #[serde(rename = "aclProfileName")]
  acl_profile_name: Option<String>,
  /// The Client Profile of the Client Username. The default value is `\"default\"`.
  #[serde(rename = "clientProfileName")]
  client_profile_name: Option<String>,
  /// The value of the Client Username.
  #[serde(rename = "clientUsername")]
  client_username: Option<String>,
  /// Enables or disables the Client Username. When disabled all clients currently connected as the Client Username are disconnected. The default value is `false`.
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  /// Enables or disables guaranteed endpoint permission override for the Client Username. When enabled all guaranteed endpoints may be accessed, modified or deleted with the same permission as the owner. The default value is `false`.
  #[serde(rename = "guaranteedEndpointPermissionOverrideEnabled")]
  guaranteed_endpoint_permission_override_enabled: Option<bool>,
  /// The name of the Message VPN.
  #[serde(rename = "msgVpnName")]
  msg_vpn_name: Option<String>,
  /// The password of this Client Username for internal Authentication. The default is to have no password. The default is to have no `password`.
  #[serde(rename = "password")]
  password: Option<String>,
  /// Enables or disables the subscription management capability of the Client Username. This is the ability to manage subscriptions on behalf of other Client Usernames. The default value is `false`.
  #[serde(rename = "subscriptionManagerEnabled")]
  subscription_manager_enabled: Option<bool>
}

impl MsgVpnClientUsername {
  pub fn new() -> MsgVpnClientUsername {
    MsgVpnClientUsername {
      acl_profile_name: None,
      client_profile_name: None,
      client_username: None,
      enabled: None,
      guaranteed_endpoint_permission_override_enabled: None,
      msg_vpn_name: None,
      password: None,
      subscription_manager_enabled: None
    }
  }

  pub fn set_acl_profile_name(&mut self, acl_profile_name: String) {
    self.acl_profile_name = Some(acl_profile_name);
  }

  pub fn with_acl_profile_name(mut self, acl_profile_name: String) -> MsgVpnClientUsername {
    self.acl_profile_name = Some(acl_profile_name);
    self
  }

  pub fn acl_profile_name(&self) -> Option<&String> {
    self.acl_profile_name.as_ref()
  }

  pub fn reset_acl_profile_name(&mut self) {
    self.acl_profile_name = None;
  }

  pub fn set_client_profile_name(&mut self, client_profile_name: String) {
    self.client_profile_name = Some(client_profile_name);
  }

  pub fn with_client_profile_name(mut self, client_profile_name: String) -> MsgVpnClientUsername {
    self.client_profile_name = Some(client_profile_name);
    self
  }

  pub fn client_profile_name(&self) -> Option<&String> {
    self.client_profile_name.as_ref()
  }

  pub fn reset_client_profile_name(&mut self) {
    self.client_profile_name = None;
  }

  pub fn set_client_username(&mut self, client_username: String) {
    self.client_username = Some(client_username);
  }

  pub fn with_client_username(mut self, client_username: String) -> MsgVpnClientUsername {
    self.client_username = Some(client_username);
    self
  }

  pub fn client_username(&self) -> Option<&String> {
    self.client_username.as_ref()
  }

  pub fn reset_client_username(&mut self) {
    self.client_username = None;
  }

  pub fn set_enabled(&mut self, enabled: bool) {
    self.enabled = Some(enabled);
  }

  pub fn with_enabled(mut self, enabled: bool) -> MsgVpnClientUsername {
    self.enabled = Some(enabled);
    self
  }

  pub fn enabled(&self) -> Option<&bool> {
    self.enabled.as_ref()
  }

  pub fn reset_enabled(&mut self) {
    self.enabled = None;
  }

  pub fn set_guaranteed_endpoint_permission_override_enabled(&mut self, guaranteed_endpoint_permission_override_enabled: bool) {
    self.guaranteed_endpoint_permission_override_enabled = Some(guaranteed_endpoint_permission_override_enabled);
  }

  pub fn with_guaranteed_endpoint_permission_override_enabled(mut self, guaranteed_endpoint_permission_override_enabled: bool) -> MsgVpnClientUsername {
    self.guaranteed_endpoint_permission_override_enabled = Some(guaranteed_endpoint_permission_override_enabled);
    self
  }

  pub fn guaranteed_endpoint_permission_override_enabled(&self) -> Option<&bool> {
    self.guaranteed_endpoint_permission_override_enabled.as_ref()
  }

  pub fn reset_guaranteed_endpoint_permission_override_enabled(&mut self) {
    self.guaranteed_endpoint_permission_override_enabled = None;
  }

  pub fn set_msg_vpn_name(&mut self, msg_vpn_name: String) {
    self.msg_vpn_name = Some(msg_vpn_name);
  }

  pub fn with_msg_vpn_name(mut self, msg_vpn_name: String) -> MsgVpnClientUsername {
    self.msg_vpn_name = Some(msg_vpn_name);
    self
  }

  pub fn msg_vpn_name(&self) -> Option<&String> {
    self.msg_vpn_name.as_ref()
  }

  pub fn reset_msg_vpn_name(&mut self) {
    self.msg_vpn_name = None;
  }

  pub fn set_password(&mut self, password: String) {
    self.password = Some(password);
  }

  pub fn with_password(mut self, password: String) -> MsgVpnClientUsername {
    self.password = Some(password);
    self
  }

  pub fn password(&self) -> Option<&String> {
    self.password.as_ref()
  }

  pub fn reset_password(&mut self) {
    self.password = None;
  }

  pub fn set_subscription_manager_enabled(&mut self, subscription_manager_enabled: bool) {
    self.subscription_manager_enabled = Some(subscription_manager_enabled);
  }

  pub fn with_subscription_manager_enabled(mut self, subscription_manager_enabled: bool) -> MsgVpnClientUsername {
    self.subscription_manager_enabled = Some(subscription_manager_enabled);
    self
  }

  pub fn subscription_manager_enabled(&self) -> Option<&bool> {
    self.subscription_manager_enabled.as_ref()
  }

  pub fn reset_subscription_manager_enabled(&mut self) {
    self.subscription_manager_enabled = None;
  }

}



