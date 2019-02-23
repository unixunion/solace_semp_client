# Rust API client for solace_semp_client

SEMP (starting in `v2`, see [note 1](#notes)) is a RESTful API for configuring, monitoring, and administering a Solace PubSub+ broker.  SEMP uses URIs to address manageable **resources** of the Solace PubSub+  broker. Resources are either individual **objects**, or **collections** of  objects. This document applies to the following API:   API|Base Path|Purpose|Comments :---|:---|:---|:--- Configuration|/SEMP/v2/config|Reading and writing config state|See [note 2](#notes)    Resources are always nouns, with individual objects being singular and  collections being plural. Objects within a collection are identified by an  `obj-id`, which follows the collection name with the form  `collection-name/obj-id`. Some examples:  <pre> /SEMP/v2/config/msgVpns                       ; MsgVpn collection /SEMP/v2/config/msgVpns/finance               ; MsgVpn object named \"finance\" /SEMP/v2/config/msgVpns/finance/queues        ; Queue collection within MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance/queues/orderQ ; Queue object named \"orderQ\" within MsgVpn \"finance\" </pre>  ## Collection Resources  Collections are unordered lists of objects (unless described as otherwise), and  are described by JSON arrays. Each item in the array represents an object in  the same manner as the individual object would normally be represented. The creation of a new object is done through its collection  resource.   ## Object Resources  Objects are composed of attributes and collections, and are described by JSON  content as name/value pairs. The collections of an object are not contained  directly in the object's JSON content, rather the content includes a URI  attribute which points to the collection. This contained collection resource  must be managed as a separate resource through this URI.  At a minimum, every object has 1 or more identifying attributes, and its own  `uri` attribute which contains the URI to itself. Attributes may have any  (non-exclusively) of the following properties:   Property|Meaning|Comments :---|:---|:--- Identifying|Attribute is involved in unique identification of the object, and appears in its URI| Required|Attribute must be provided in the request| Read-Only|Attribute can only be read, not written|See [note 3](#notes) Write-Only|Attribute can only be written, not read| Requires-Disable|Attribute can only be changed when object is disabled| Deprecated|Attribute is deprecated, and will disappear in the next SEMP version|    In some requests, certain attributes may only be provided in  certain combinations with other attributes:   Relationship|Meaning :---|:--- Requires|Attribute may only be changed by a request if a particular attribute or combination of attributes is also provided in the request Conflicts|Attribute may only be provided in a request if a particular attribute or combination of attributes is not also provided in the request     ## HTTP Methods  The following HTTP methods manipulate resources in accordance with these  general principles:   Method|Resource|Meaning|Request Body|Response Body|Missing Request Attributes :---|:---|:---|:---|:---|:--- POST|Collection|Create object|Initial attribute values|Object attributes and metadata|Set to default PUT|Object|Create or replace object|New attribute values|Object attributes and metadata|Set to default (but see [note 4](#notes)) PATCH|Object|Update object|New attribute values|Object attributes and metadata|unchanged DELETE|Object|Delete object|Empty|Object metadata|N/A GET|Object|Get object|Empty|Object attributes and metadata|N/A GET|Collection|Get collection|Empty|Object attributes and collection metadata|N/A    ## Common Query Parameters  The following are some common query parameters that are supported by many  method/URI combinations. Individual URIs may document additional parameters.  Note that multiple query parameters can be used together in a single URI,  separated by the ampersand character. For example:  <pre> ; Request for the MsgVpns collection using two hypothetical query parameters ; \"q1\" and \"q2\" with values \"val1\" and \"val2\" respectively /SEMP/v2/config/msgVpns?q1=val1&q2=val2 </pre>  ### select  Include in the response only selected attributes of the object, or exclude  from the response selected attributes of the object. Use this query parameter  to limit the size of the returned data for each returned object, return only  those fields that are desired, or exclude fields that are not desired.  The value of `select` is a comma-separated list of attribute names. If the  list contains attribute names that are not prefaced by `-`, only those  attributes are included in the response. If the list contains attribute names  that are prefaced by `-`, those attributes are excluded from the response. If  the list contains both types, then the difference of the first set of  attributes and the second set of attributes is returned. If the list is  empty (i.e. `select=`), no attributes are returned  All attributes that are prefaced by `-` must follow all attributes that are  not prefaced by `-`. In addition, each attribute name in the list must match  at least one attribute in the object.  Names may include the `*` wildcard (zero or more characters). Nested attribute  names are supported using periods (e.g. `parentName.childName`).  Some examples:  <pre> ; List of all MsgVpn names /SEMP/v2/config/msgVpns?select=msgVpnName  ; List of all MsgVpn and their attributes except for their names /SEMP/v2/config/msgVpns?select=-msgVpnName  ; Authentication attributes of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance?select=authentication*  ; All attributes of MsgVpn \"finance\" except for authentication attributes /SEMP/v2/config/msgVpns/finance?select=-authentication*  ; Access related attributes of Queue \"orderQ\" of MsgVpn \"finance\" /SEMP/v2/config/msgVpns/finance/queues/orderQ?select=owner,permission </pre>  ### where  Include in the response only objects where certain conditions are true. Use  this query parameter to limit which objects are returned to those whose  attribute values meet the given conditions.  The value of `where` is a comma-separated list of expressions. All expressions  must be true for the object to be included in the response. Each expression  takes the form:  <pre> expression  = attribute-name OP value OP          = '==' | '!=' | '&lt;' | '&gt;' | '&lt;=' | '&gt;=' </pre>  `value` may be a number, string, `true`, or `false`, as appropriate for the  type of `attribute-name`. Greater-than and less-than comparisons only work for  numbers. A `*` in a string `value` is interpreted as a wildcard (zero or more  characters). Some examples:  <pre> ; Only enabled MsgVpns /SEMP/v2/config/msgVpns?where=enabled==true  ; Only MsgVpns using basic non-LDAP authentication /SEMP/v2/config/msgVpns?where=authenticationBasicEnabled==true,authenticationBasicType!=ldap  ; Only MsgVpns that allow more than 100 client connections /SEMP/v2/config/msgVpns?where=maxConnectionCount>100  ; Only MsgVpns with msgVpnName starting with \"B\": /SEMP/v2/config/msgVpns?where=msgVpnName==B* </pre>  ### count  Limit the count of objects in the response. This can be useful to limit the  size of the response for large collections. The minimum value for `count` is  `1` and the default is `10`. There is a hidden maximum  as to prevent overloading the system. For example:  <pre> ; Up to 25 MsgVpns /SEMP/v2/config/msgVpns?count=25 </pre>  ### cursor  The cursor, or position, for the next page of objects. Cursors are opaque data  that should not be created or interpreted by SEMP clients, and should only be  used as described below.  When a request is made for a collection and there may be additional objects  available for retrieval that are not included in the initial response, the  response will include a `cursorQuery` field containing a cursor. The value  of this field can be specified in the `cursor` query parameter of a  subsequent request to retrieve the next page of objects. For convenience,  an appropriate URI is constructed automatically by the broker and included  in the `nextPageUri` field of the response. This URI can be used directly  to retrieve the next page of objects.  ## Notes  Note|Description :---:|:--- 1|This specification defines SEMP starting in \"v2\", and not the original SEMP \"v1\" interface. Request and response formats between \"v1\" and \"v2\" are entirely incompatible, although both protocols share a common port configuration on the Solace PubSub+ broker. They are differentiated by the initial portion of the URI path, one of either \"/SEMP/\" or \"/SEMP/v2/\" 2|This API is partially implemented. Only a subset of all objects are available. 3|Read-only attributes may appear in POST and PUT/PATCH requests. However, if a read-only attribute is not marked as identifying, it will be ignored during a PUT/PATCH. 4|For PUT, if the SEMP user is not authorized to modify the attribute, its value is left unchanged rather than set to default. In addition, the values of write-only attributes are not set to their defaults on a PUT. If the object does not exist, it is created first. 5|For DELETE, the body of the request currently serves no purpose and will cause an error if not empty.    

## Overview
This API client was generated by the [swagger-codegen](https://github.com/swagger-api/swagger-codegen) project.  By using the [swagger-spec](https://github.com/swagger-api/swagger-spec) from a remote server, you can easily generate an API client.

- API version: 2.10
- Package version: 9.0.1.7
- Build package: io.swagger.codegen.languages.RustClientCodegen
For more information, please visit [http://www.solace.com](http://www.solace.com)

## Installation
Put the package under your project folder and add the following in import:
```
    "./solace_semp_client"
```

## Documentation for API Endpoints

All URIs are relative to *http://www.solace.com/SEMP/v2/config*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AboutApi* | [**get_about_api**](docs/AboutApi.md#get_about_api) | **Get** /about/api | Gets an API Description object.
*AboutApi* | [**get_about_user**](docs/AboutApi.md#get_about_user) | **Get** /about/user | Gets a Current User object.
*AclProfileApi* | [**get_msg_vpn_acl_profiles**](docs/AclProfileApi.md#get_msg_vpn_acl_profiles) | **Get** /msgVpns/{msgVpnName}/aclProfiles | Gets a list of ACL Profile objects.
*AuthorizationGroupApi* | [**get_msg_vpn_authorization_groups**](docs/AuthorizationGroupApi.md#get_msg_vpn_authorization_groups) | **Get** /msgVpns/{msgVpnName}/authorizationGroups | Gets a list of LDAP Authorization Group objects.
*BridgeApi* | [**get_msg_vpn_bridges**](docs/BridgeApi.md#get_msg_vpn_bridges) | **Get** /msgVpns/{msgVpnName}/bridges | Gets a list of Bridge objects.
*ClientProfileApi* | [**get_msg_vpn_client_profiles**](docs/ClientProfileApi.md#get_msg_vpn_client_profiles) | **Get** /msgVpns/{msgVpnName}/clientProfiles | Gets a list of Client Profile objects.
*ClientUsernameApi* | [**get_msg_vpn_client_usernames**](docs/ClientUsernameApi.md#get_msg_vpn_client_usernames) | **Get** /msgVpns/{msgVpnName}/clientUsernames | Gets a list of Client Username objects.
*DefaultApi* | [**create_msg_vpn**](docs/DefaultApi.md#create_msg_vpn) | **Post** /msgVpns | Creates a Message VPN object.
*DefaultApi* | [**create_msg_vpn_acl_profile**](docs/DefaultApi.md#create_msg_vpn_acl_profile) | **Post** /msgVpns/{msgVpnName}/aclProfiles | Creates an ACL Profile object.
*DefaultApi* | [**create_msg_vpn_acl_profile_client_connect_exception**](docs/DefaultApi.md#create_msg_vpn_acl_profile_client_connect_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions | Creates a Client Connect Exception object.
*DefaultApi* | [**create_msg_vpn_acl_profile_publish_exception**](docs/DefaultApi.md#create_msg_vpn_acl_profile_publish_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions | Creates a Publish Topic Exception object.
*DefaultApi* | [**create_msg_vpn_acl_profile_subscribe_exception**](docs/DefaultApi.md#create_msg_vpn_acl_profile_subscribe_exception) | **Post** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions | Creates a Subscribe Topic Exception object.
*DefaultApi* | [**create_msg_vpn_authorization_group**](docs/DefaultApi.md#create_msg_vpn_authorization_group) | **Post** /msgVpns/{msgVpnName}/authorizationGroups | Creates an LDAP Authorization Group object.
*DefaultApi* | [**create_msg_vpn_bridge**](docs/DefaultApi.md#create_msg_vpn_bridge) | **Post** /msgVpns/{msgVpnName}/bridges | Creates a Bridge object.
*DefaultApi* | [**create_msg_vpn_bridge_remote_msg_vpn**](docs/DefaultApi.md#create_msg_vpn_bridge_remote_msg_vpn) | **Post** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns | Creates a Remote Message VPN object.
*DefaultApi* | [**create_msg_vpn_bridge_remote_subscription**](docs/DefaultApi.md#create_msg_vpn_bridge_remote_subscription) | **Post** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions | Creates a Remote Subscription object.
*DefaultApi* | [**create_msg_vpn_bridge_tls_trusted_common_name**](docs/DefaultApi.md#create_msg_vpn_bridge_tls_trusted_common_name) | **Post** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames | Creates a Trusted Common Name object.
*DefaultApi* | [**create_msg_vpn_client_profile**](docs/DefaultApi.md#create_msg_vpn_client_profile) | **Post** /msgVpns/{msgVpnName}/clientProfiles | Creates a Client Profile object.
*DefaultApi* | [**create_msg_vpn_client_username**](docs/DefaultApi.md#create_msg_vpn_client_username) | **Post** /msgVpns/{msgVpnName}/clientUsernames | Creates a Client Username object.
*DefaultApi* | [**create_msg_vpn_jndi_connection_factory**](docs/DefaultApi.md#create_msg_vpn_jndi_connection_factory) | **Post** /msgVpns/{msgVpnName}/jndiConnectionFactories | Creates a JNDI Connection Factory object.
*DefaultApi* | [**create_msg_vpn_jndi_queue**](docs/DefaultApi.md#create_msg_vpn_jndi_queue) | **Post** /msgVpns/{msgVpnName}/jndiQueues | Creates a JNDI Queue object.
*DefaultApi* | [**create_msg_vpn_jndi_topic**](docs/DefaultApi.md#create_msg_vpn_jndi_topic) | **Post** /msgVpns/{msgVpnName}/jndiTopics | Creates a JNDI Topic object.
*DefaultApi* | [**create_msg_vpn_mqtt_session**](docs/DefaultApi.md#create_msg_vpn_mqtt_session) | **Post** /msgVpns/{msgVpnName}/mqttSessions | Creates an MQTT Session object.
*DefaultApi* | [**create_msg_vpn_mqtt_session_subscription**](docs/DefaultApi.md#create_msg_vpn_mqtt_session_subscription) | **Post** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions | Creates an MQTT Session Subscription object.
*DefaultApi* | [**create_msg_vpn_queue**](docs/DefaultApi.md#create_msg_vpn_queue) | **Post** /msgVpns/{msgVpnName}/queues | Creates a Queue object.
*DefaultApi* | [**create_msg_vpn_queue_subscription**](docs/DefaultApi.md#create_msg_vpn_queue_subscription) | **Post** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions | Creates a Queue Subscription object.
*DefaultApi* | [**create_msg_vpn_replay_log**](docs/DefaultApi.md#create_msg_vpn_replay_log) | **Post** /msgVpns/{msgVpnName}/replayLogs | Creates a ReplayLog object.
*DefaultApi* | [**create_msg_vpn_replicated_topic**](docs/DefaultApi.md#create_msg_vpn_replicated_topic) | **Post** /msgVpns/{msgVpnName}/replicatedTopics | Creates a Replicated Topic object.
*DefaultApi* | [**create_msg_vpn_rest_delivery_point**](docs/DefaultApi.md#create_msg_vpn_rest_delivery_point) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints | Creates a REST Delivery Point object.
*DefaultApi* | [**create_msg_vpn_rest_delivery_point_queue_binding**](docs/DefaultApi.md#create_msg_vpn_rest_delivery_point_queue_binding) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings | Creates a Queue Binding object.
*DefaultApi* | [**create_msg_vpn_rest_delivery_point_rest_consumer**](docs/DefaultApi.md#create_msg_vpn_rest_delivery_point_rest_consumer) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers | Creates a REST Consumer object.
*DefaultApi* | [**create_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](docs/DefaultApi.md#create_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Post** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames | Creates a Trusted Common Name object.
*DefaultApi* | [**create_msg_vpn_sequenced_topic**](docs/DefaultApi.md#create_msg_vpn_sequenced_topic) | **Post** /msgVpns/{msgVpnName}/sequencedTopics | Creates a Sequenced Topic object.
*DefaultApi* | [**create_msg_vpn_topic_endpoint**](docs/DefaultApi.md#create_msg_vpn_topic_endpoint) | **Post** /msgVpns/{msgVpnName}/topicEndpoints | Creates a Topic Endpoint object.
*DefaultApi* | [**delete_msg_vpn**](docs/DefaultApi.md#delete_msg_vpn) | **Delete** /msgVpns/{msgVpnName} | Deletes a Message VPN object.
*DefaultApi* | [**delete_msg_vpn_acl_profile**](docs/DefaultApi.md#delete_msg_vpn_acl_profile) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Deletes an ACL Profile object.
*DefaultApi* | [**delete_msg_vpn_acl_profile_client_connect_exception**](docs/DefaultApi.md#delete_msg_vpn_acl_profile_client_connect_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions/{clientConnectExceptionAddress} | Deletes a Client Connect Exception object.
*DefaultApi* | [**delete_msg_vpn_acl_profile_publish_exception**](docs/DefaultApi.md#delete_msg_vpn_acl_profile_publish_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions/{topicSyntax},{publishExceptionTopic} | Deletes a Publish Topic Exception object.
*DefaultApi* | [**delete_msg_vpn_acl_profile_subscribe_exception**](docs/DefaultApi.md#delete_msg_vpn_acl_profile_subscribe_exception) | **Delete** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions/{topicSyntax},{subscribeExceptionTopic} | Deletes a Subscribe Topic Exception object.
*DefaultApi* | [**delete_msg_vpn_authorization_group**](docs/DefaultApi.md#delete_msg_vpn_authorization_group) | **Delete** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Deletes an LDAP Authorization Group object.
*DefaultApi* | [**delete_msg_vpn_bridge**](docs/DefaultApi.md#delete_msg_vpn_bridge) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Deletes a Bridge object.
*DefaultApi* | [**delete_msg_vpn_bridge_remote_msg_vpn**](docs/DefaultApi.md#delete_msg_vpn_bridge_remote_msg_vpn) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Deletes a Remote Message VPN object.
*DefaultApi* | [**delete_msg_vpn_bridge_remote_subscription**](docs/DefaultApi.md#delete_msg_vpn_bridge_remote_subscription) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions/{remoteSubscriptionTopic} | Deletes a Remote Subscription object.
*DefaultApi* | [**delete_msg_vpn_bridge_tls_trusted_common_name**](docs/DefaultApi.md#delete_msg_vpn_bridge_tls_trusted_common_name) | **Delete** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Deletes a Trusted Common Name object.
*DefaultApi* | [**delete_msg_vpn_client_profile**](docs/DefaultApi.md#delete_msg_vpn_client_profile) | **Delete** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Deletes a Client Profile object.
*DefaultApi* | [**delete_msg_vpn_client_username**](docs/DefaultApi.md#delete_msg_vpn_client_username) | **Delete** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Deletes a Client Username object.
*DefaultApi* | [**delete_msg_vpn_jndi_connection_factory**](docs/DefaultApi.md#delete_msg_vpn_jndi_connection_factory) | **Delete** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Deletes a JNDI Connection Factory object.
*DefaultApi* | [**delete_msg_vpn_jndi_queue**](docs/DefaultApi.md#delete_msg_vpn_jndi_queue) | **Delete** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Deletes a JNDI Queue object.
*DefaultApi* | [**delete_msg_vpn_jndi_topic**](docs/DefaultApi.md#delete_msg_vpn_jndi_topic) | **Delete** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Deletes a JNDI Topic object.
*DefaultApi* | [**delete_msg_vpn_mqtt_session**](docs/DefaultApi.md#delete_msg_vpn_mqtt_session) | **Delete** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Deletes an MQTT Session object.
*DefaultApi* | [**delete_msg_vpn_mqtt_session_subscription**](docs/DefaultApi.md#delete_msg_vpn_mqtt_session_subscription) | **Delete** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Deletes an MQTT Session Subscription object.
*DefaultApi* | [**delete_msg_vpn_queue**](docs/DefaultApi.md#delete_msg_vpn_queue) | **Delete** /msgVpns/{msgVpnName}/queues/{queueName} | Deletes a Queue object.
*DefaultApi* | [**delete_msg_vpn_queue_subscription**](docs/DefaultApi.md#delete_msg_vpn_queue_subscription) | **Delete** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions/{subscriptionTopic} | Deletes a Queue Subscription object.
*DefaultApi* | [**delete_msg_vpn_replay_log**](docs/DefaultApi.md#delete_msg_vpn_replay_log) | **Delete** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Deletes a ReplayLog object.
*DefaultApi* | [**delete_msg_vpn_replicated_topic**](docs/DefaultApi.md#delete_msg_vpn_replicated_topic) | **Delete** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Deletes a Replicated Topic object.
*DefaultApi* | [**delete_msg_vpn_rest_delivery_point**](docs/DefaultApi.md#delete_msg_vpn_rest_delivery_point) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Deletes a REST Delivery Point object.
*DefaultApi* | [**delete_msg_vpn_rest_delivery_point_queue_binding**](docs/DefaultApi.md#delete_msg_vpn_rest_delivery_point_queue_binding) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Deletes a Queue Binding object.
*DefaultApi* | [**delete_msg_vpn_rest_delivery_point_rest_consumer**](docs/DefaultApi.md#delete_msg_vpn_rest_delivery_point_rest_consumer) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Deletes a REST Consumer object.
*DefaultApi* | [**delete_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](docs/DefaultApi.md#delete_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Delete** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Deletes a Trusted Common Name object.
*DefaultApi* | [**delete_msg_vpn_sequenced_topic**](docs/DefaultApi.md#delete_msg_vpn_sequenced_topic) | **Delete** /msgVpns/{msgVpnName}/sequencedTopics/{sequencedTopic} | Deletes a Sequenced Topic object.
*DefaultApi* | [**delete_msg_vpn_topic_endpoint**](docs/DefaultApi.md#delete_msg_vpn_topic_endpoint) | **Delete** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Deletes a Topic Endpoint object.
*DefaultApi* | [**get_about_user_msg_vpn**](docs/DefaultApi.md#get_about_user_msg_vpn) | **Get** /about/user/msgVpns/{msgVpnName} | Gets a Current User Message VPN object.
*DefaultApi* | [**get_about_user_msg_vpns**](docs/DefaultApi.md#get_about_user_msg_vpns) | **Get** /about/user/msgVpns | Gets a list of Current User Message VPN objects.
*DefaultApi* | [**get_msg_vpn**](docs/DefaultApi.md#get_msg_vpn) | **Get** /msgVpns/{msgVpnName} | Gets a Message VPN object.
*DefaultApi* | [**get_msg_vpn_acl_profile**](docs/DefaultApi.md#get_msg_vpn_acl_profile) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Gets an ACL Profile object.
*DefaultApi* | [**get_msg_vpn_acl_profile_client_connect_exception**](docs/DefaultApi.md#get_msg_vpn_acl_profile_client_connect_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions/{clientConnectExceptionAddress} | Gets a Client Connect Exception object.
*DefaultApi* | [**get_msg_vpn_acl_profile_client_connect_exceptions**](docs/DefaultApi.md#get_msg_vpn_acl_profile_client_connect_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/clientConnectExceptions | Gets a list of Client Connect Exception objects.
*DefaultApi* | [**get_msg_vpn_acl_profile_publish_exception**](docs/DefaultApi.md#get_msg_vpn_acl_profile_publish_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions/{topicSyntax},{publishExceptionTopic} | Gets a Publish Topic Exception object.
*DefaultApi* | [**get_msg_vpn_acl_profile_publish_exceptions**](docs/DefaultApi.md#get_msg_vpn_acl_profile_publish_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/publishExceptions | Gets a list of Publish Topic Exception objects.
*DefaultApi* | [**get_msg_vpn_acl_profile_subscribe_exception**](docs/DefaultApi.md#get_msg_vpn_acl_profile_subscribe_exception) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions/{topicSyntax},{subscribeExceptionTopic} | Gets a Subscribe Topic Exception object.
*DefaultApi* | [**get_msg_vpn_acl_profile_subscribe_exceptions**](docs/DefaultApi.md#get_msg_vpn_acl_profile_subscribe_exceptions) | **Get** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName}/subscribeExceptions | Gets a list of Subscribe Topic Exception objects.
*DefaultApi* | [**get_msg_vpn_authorization_group**](docs/DefaultApi.md#get_msg_vpn_authorization_group) | **Get** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Gets an LDAP Authorization Group object.
*DefaultApi* | [**get_msg_vpn_bridge**](docs/DefaultApi.md#get_msg_vpn_bridge) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Gets a Bridge object.
*DefaultApi* | [**get_msg_vpn_bridge_remote_msg_vpn**](docs/DefaultApi.md#get_msg_vpn_bridge_remote_msg_vpn) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Gets a Remote Message VPN object.
*DefaultApi* | [**get_msg_vpn_bridge_remote_msg_vpns**](docs/DefaultApi.md#get_msg_vpn_bridge_remote_msg_vpns) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns | Gets a list of Remote Message VPN objects.
*DefaultApi* | [**get_msg_vpn_bridge_remote_subscription**](docs/DefaultApi.md#get_msg_vpn_bridge_remote_subscription) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions/{remoteSubscriptionTopic} | Gets a Remote Subscription object.
*DefaultApi* | [**get_msg_vpn_bridge_remote_subscriptions**](docs/DefaultApi.md#get_msg_vpn_bridge_remote_subscriptions) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteSubscriptions | Gets a list of Remote Subscription objects.
*DefaultApi* | [**get_msg_vpn_bridge_tls_trusted_common_name**](docs/DefaultApi.md#get_msg_vpn_bridge_tls_trusted_common_name) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Gets a Trusted Common Name object.
*DefaultApi* | [**get_msg_vpn_bridge_tls_trusted_common_names**](docs/DefaultApi.md#get_msg_vpn_bridge_tls_trusted_common_names) | **Get** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/tlsTrustedCommonNames | Gets a list of Trusted Common Name objects.
*DefaultApi* | [**get_msg_vpn_client_profile**](docs/DefaultApi.md#get_msg_vpn_client_profile) | **Get** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Gets a Client Profile object.
*DefaultApi* | [**get_msg_vpn_client_username**](docs/DefaultApi.md#get_msg_vpn_client_username) | **Get** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Gets a Client Username object.
*DefaultApi* | [**get_msg_vpn_jndi_connection_factory**](docs/DefaultApi.md#get_msg_vpn_jndi_connection_factory) | **Get** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Gets a JNDI Connection Factory object.
*DefaultApi* | [**get_msg_vpn_jndi_queue**](docs/DefaultApi.md#get_msg_vpn_jndi_queue) | **Get** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Gets a JNDI Queue object.
*DefaultApi* | [**get_msg_vpn_jndi_topic**](docs/DefaultApi.md#get_msg_vpn_jndi_topic) | **Get** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Gets a JNDI Topic object.
*DefaultApi* | [**get_msg_vpn_mqtt_session**](docs/DefaultApi.md#get_msg_vpn_mqtt_session) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Gets an MQTT Session object.
*DefaultApi* | [**get_msg_vpn_mqtt_session_subscription**](docs/DefaultApi.md#get_msg_vpn_mqtt_session_subscription) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Gets an MQTT Session Subscription object.
*DefaultApi* | [**get_msg_vpn_mqtt_session_subscriptions**](docs/DefaultApi.md#get_msg_vpn_mqtt_session_subscriptions) | **Get** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions | Gets a list of MQTT Session Subscription objects.
*DefaultApi* | [**get_msg_vpn_queue**](docs/DefaultApi.md#get_msg_vpn_queue) | **Get** /msgVpns/{msgVpnName}/queues/{queueName} | Gets a Queue object.
*DefaultApi* | [**get_msg_vpn_queue_subscription**](docs/DefaultApi.md#get_msg_vpn_queue_subscription) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions/{subscriptionTopic} | Gets a Queue Subscription object.
*DefaultApi* | [**get_msg_vpn_queue_subscriptions**](docs/DefaultApi.md#get_msg_vpn_queue_subscriptions) | **Get** /msgVpns/{msgVpnName}/queues/{queueName}/subscriptions | Gets a list of Queue Subscription objects.
*DefaultApi* | [**get_msg_vpn_replay_log**](docs/DefaultApi.md#get_msg_vpn_replay_log) | **Get** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Gets a ReplayLog object.
*DefaultApi* | [**get_msg_vpn_replicated_topic**](docs/DefaultApi.md#get_msg_vpn_replicated_topic) | **Get** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Gets a Replicated Topic object.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Gets a REST Delivery Point object.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point_queue_binding**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point_queue_binding) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Gets a Queue Binding object.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point_queue_bindings**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point_queue_bindings) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings | Gets a list of Queue Binding objects.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumer) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Gets a REST Consumer object.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_name) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames/{tlsTrustedCommonName} | Gets a Trusted Common Name object.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumer_tls_trusted_common_names) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName}/tlsTrustedCommonNames | Gets a list of Trusted Common Name objects.
*DefaultApi* | [**get_msg_vpn_rest_delivery_point_rest_consumers**](docs/DefaultApi.md#get_msg_vpn_rest_delivery_point_rest_consumers) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers | Gets a list of REST Consumer objects.
*DefaultApi* | [**get_msg_vpn_sequenced_topic**](docs/DefaultApi.md#get_msg_vpn_sequenced_topic) | **Get** /msgVpns/{msgVpnName}/sequencedTopics/{sequencedTopic} | Gets a Sequenced Topic object.
*DefaultApi* | [**get_msg_vpn_sequenced_topics**](docs/DefaultApi.md#get_msg_vpn_sequenced_topics) | **Get** /msgVpns/{msgVpnName}/sequencedTopics | Gets a list of Sequenced Topic objects.
*DefaultApi* | [**get_msg_vpn_topic_endpoint**](docs/DefaultApi.md#get_msg_vpn_topic_endpoint) | **Get** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Gets a Topic Endpoint object.
*DefaultApi* | [**replace_msg_vpn**](docs/DefaultApi.md#replace_msg_vpn) | **Put** /msgVpns/{msgVpnName} | Replaces a Message VPN object.
*DefaultApi* | [**replace_msg_vpn_acl_profile**](docs/DefaultApi.md#replace_msg_vpn_acl_profile) | **Put** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Replaces an ACL Profile object.
*DefaultApi* | [**replace_msg_vpn_authorization_group**](docs/DefaultApi.md#replace_msg_vpn_authorization_group) | **Put** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Replaces an LDAP Authorization Group object.
*DefaultApi* | [**replace_msg_vpn_bridge**](docs/DefaultApi.md#replace_msg_vpn_bridge) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Replaces a Bridge object.
*DefaultApi* | [**replace_msg_vpn_bridge_remote_msg_vpn**](docs/DefaultApi.md#replace_msg_vpn_bridge_remote_msg_vpn) | **Put** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Replaces a Remote Message VPN object.
*DefaultApi* | [**replace_msg_vpn_client_profile**](docs/DefaultApi.md#replace_msg_vpn_client_profile) | **Put** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Replaces a Client Profile object.
*DefaultApi* | [**replace_msg_vpn_client_username**](docs/DefaultApi.md#replace_msg_vpn_client_username) | **Put** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Replaces a Client Username object.
*DefaultApi* | [**replace_msg_vpn_jndi_connection_factory**](docs/DefaultApi.md#replace_msg_vpn_jndi_connection_factory) | **Put** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Replaces a JNDI Connection Factory object.
*DefaultApi* | [**replace_msg_vpn_jndi_queue**](docs/DefaultApi.md#replace_msg_vpn_jndi_queue) | **Put** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Replaces a JNDI Queue object.
*DefaultApi* | [**replace_msg_vpn_jndi_topic**](docs/DefaultApi.md#replace_msg_vpn_jndi_topic) | **Put** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Replaces a JNDI Topic object.
*DefaultApi* | [**replace_msg_vpn_mqtt_session**](docs/DefaultApi.md#replace_msg_vpn_mqtt_session) | **Put** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Replaces an MQTT Session object.
*DefaultApi* | [**replace_msg_vpn_mqtt_session_subscription**](docs/DefaultApi.md#replace_msg_vpn_mqtt_session_subscription) | **Put** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Replaces an MQTT Session Subscription object.
*DefaultApi* | [**replace_msg_vpn_queue**](docs/DefaultApi.md#replace_msg_vpn_queue) | **Put** /msgVpns/{msgVpnName}/queues/{queueName} | Replaces a Queue object.
*DefaultApi* | [**replace_msg_vpn_replay_log**](docs/DefaultApi.md#replace_msg_vpn_replay_log) | **Put** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Replaces a ReplayLog object.
*DefaultApi* | [**replace_msg_vpn_replicated_topic**](docs/DefaultApi.md#replace_msg_vpn_replicated_topic) | **Put** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Replaces a Replicated Topic object.
*DefaultApi* | [**replace_msg_vpn_rest_delivery_point**](docs/DefaultApi.md#replace_msg_vpn_rest_delivery_point) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Replaces a REST Delivery Point object.
*DefaultApi* | [**replace_msg_vpn_rest_delivery_point_queue_binding**](docs/DefaultApi.md#replace_msg_vpn_rest_delivery_point_queue_binding) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Replaces a Queue Binding object.
*DefaultApi* | [**replace_msg_vpn_rest_delivery_point_rest_consumer**](docs/DefaultApi.md#replace_msg_vpn_rest_delivery_point_rest_consumer) | **Put** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Replaces a REST Consumer object.
*DefaultApi* | [**replace_msg_vpn_topic_endpoint**](docs/DefaultApi.md#replace_msg_vpn_topic_endpoint) | **Put** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Replaces a Topic Endpoint object.
*DefaultApi* | [**update_msg_vpn**](docs/DefaultApi.md#update_msg_vpn) | **Patch** /msgVpns/{msgVpnName} | Updates a Message VPN object.
*DefaultApi* | [**update_msg_vpn_acl_profile**](docs/DefaultApi.md#update_msg_vpn_acl_profile) | **Patch** /msgVpns/{msgVpnName}/aclProfiles/{aclProfileName} | Updates an ACL Profile object.
*DefaultApi* | [**update_msg_vpn_authorization_group**](docs/DefaultApi.md#update_msg_vpn_authorization_group) | **Patch** /msgVpns/{msgVpnName}/authorizationGroups/{authorizationGroupName} | Updates an LDAP Authorization Group object.
*DefaultApi* | [**update_msg_vpn_bridge**](docs/DefaultApi.md#update_msg_vpn_bridge) | **Patch** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter} | Updates a Bridge object.
*DefaultApi* | [**update_msg_vpn_bridge_remote_msg_vpn**](docs/DefaultApi.md#update_msg_vpn_bridge_remote_msg_vpn) | **Patch** /msgVpns/{msgVpnName}/bridges/{bridgeName},{bridgeVirtualRouter}/remoteMsgVpns/{remoteMsgVpnName},{remoteMsgVpnLocation},{remoteMsgVpnInterface} | Updates a Remote Message VPN object.
*DefaultApi* | [**update_msg_vpn_client_profile**](docs/DefaultApi.md#update_msg_vpn_client_profile) | **Patch** /msgVpns/{msgVpnName}/clientProfiles/{clientProfileName} | Updates a Client Profile object.
*DefaultApi* | [**update_msg_vpn_client_username**](docs/DefaultApi.md#update_msg_vpn_client_username) | **Patch** /msgVpns/{msgVpnName}/clientUsernames/{clientUsername} | Updates a Client Username object.
*DefaultApi* | [**update_msg_vpn_jndi_connection_factory**](docs/DefaultApi.md#update_msg_vpn_jndi_connection_factory) | **Patch** /msgVpns/{msgVpnName}/jndiConnectionFactories/{connectionFactoryName} | Updates a JNDI Connection Factory object.
*DefaultApi* | [**update_msg_vpn_jndi_queue**](docs/DefaultApi.md#update_msg_vpn_jndi_queue) | **Patch** /msgVpns/{msgVpnName}/jndiQueues/{queueName} | Updates a JNDI Queue object.
*DefaultApi* | [**update_msg_vpn_jndi_topic**](docs/DefaultApi.md#update_msg_vpn_jndi_topic) | **Patch** /msgVpns/{msgVpnName}/jndiTopics/{topicName} | Updates a JNDI Topic object.
*DefaultApi* | [**update_msg_vpn_mqtt_session**](docs/DefaultApi.md#update_msg_vpn_mqtt_session) | **Patch** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter} | Updates an MQTT Session object.
*DefaultApi* | [**update_msg_vpn_mqtt_session_subscription**](docs/DefaultApi.md#update_msg_vpn_mqtt_session_subscription) | **Patch** /msgVpns/{msgVpnName}/mqttSessions/{mqttSessionClientId},{mqttSessionVirtualRouter}/subscriptions/{subscriptionTopic} | Updates an MQTT Session Subscription object.
*DefaultApi* | [**update_msg_vpn_queue**](docs/DefaultApi.md#update_msg_vpn_queue) | **Patch** /msgVpns/{msgVpnName}/queues/{queueName} | Updates a Queue object.
*DefaultApi* | [**update_msg_vpn_replay_log**](docs/DefaultApi.md#update_msg_vpn_replay_log) | **Patch** /msgVpns/{msgVpnName}/replayLogs/{replayLogName} | Updates a ReplayLog object.
*DefaultApi* | [**update_msg_vpn_replicated_topic**](docs/DefaultApi.md#update_msg_vpn_replicated_topic) | **Patch** /msgVpns/{msgVpnName}/replicatedTopics/{replicatedTopic} | Updates a Replicated Topic object.
*DefaultApi* | [**update_msg_vpn_rest_delivery_point**](docs/DefaultApi.md#update_msg_vpn_rest_delivery_point) | **Patch** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName} | Updates a REST Delivery Point object.
*DefaultApi* | [**update_msg_vpn_rest_delivery_point_queue_binding**](docs/DefaultApi.md#update_msg_vpn_rest_delivery_point_queue_binding) | **Patch** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/queueBindings/{queueBindingName} | Updates a Queue Binding object.
*DefaultApi* | [**update_msg_vpn_rest_delivery_point_rest_consumer**](docs/DefaultApi.md#update_msg_vpn_rest_delivery_point_rest_consumer) | **Patch** /msgVpns/{msgVpnName}/restDeliveryPoints/{restDeliveryPointName}/restConsumers/{restConsumerName} | Updates a REST Consumer object.
*DefaultApi* | [**update_msg_vpn_topic_endpoint**](docs/DefaultApi.md#update_msg_vpn_topic_endpoint) | **Patch** /msgVpns/{msgVpnName}/topicEndpoints/{topicEndpointName} | Updates a Topic Endpoint object.
*JndiApi* | [**get_msg_vpn_jndi_connection_factories**](docs/JndiApi.md#get_msg_vpn_jndi_connection_factories) | **Get** /msgVpns/{msgVpnName}/jndiConnectionFactories | Gets a list of JNDI Connection Factory objects.
*JndiApi* | [**get_msg_vpn_jndi_queues**](docs/JndiApi.md#get_msg_vpn_jndi_queues) | **Get** /msgVpns/{msgVpnName}/jndiQueues | Gets a list of JNDI Queue objects.
*JndiApi* | [**get_msg_vpn_jndi_topics**](docs/JndiApi.md#get_msg_vpn_jndi_topics) | **Get** /msgVpns/{msgVpnName}/jndiTopics | Gets a list of JNDI Topic objects.
*MqttSessionApi* | [**get_msg_vpn_mqtt_sessions**](docs/MqttSessionApi.md#get_msg_vpn_mqtt_sessions) | **Get** /msgVpns/{msgVpnName}/mqttSessions | Gets a list of MQTT Session objects.
*MsgVpnApi* | [**get_msg_vpn_acl_profiles**](docs/MsgVpnApi.md#get_msg_vpn_acl_profiles) | **Get** /msgVpns/{msgVpnName}/aclProfiles | Gets a list of ACL Profile objects.
*MsgVpnApi* | [**get_msg_vpn_authorization_groups**](docs/MsgVpnApi.md#get_msg_vpn_authorization_groups) | **Get** /msgVpns/{msgVpnName}/authorizationGroups | Gets a list of LDAP Authorization Group objects.
*MsgVpnApi* | [**get_msg_vpn_bridges**](docs/MsgVpnApi.md#get_msg_vpn_bridges) | **Get** /msgVpns/{msgVpnName}/bridges | Gets a list of Bridge objects.
*MsgVpnApi* | [**get_msg_vpn_client_profiles**](docs/MsgVpnApi.md#get_msg_vpn_client_profiles) | **Get** /msgVpns/{msgVpnName}/clientProfiles | Gets a list of Client Profile objects.
*MsgVpnApi* | [**get_msg_vpn_client_usernames**](docs/MsgVpnApi.md#get_msg_vpn_client_usernames) | **Get** /msgVpns/{msgVpnName}/clientUsernames | Gets a list of Client Username objects.
*MsgVpnApi* | [**get_msg_vpn_jndi_connection_factories**](docs/MsgVpnApi.md#get_msg_vpn_jndi_connection_factories) | **Get** /msgVpns/{msgVpnName}/jndiConnectionFactories | Gets a list of JNDI Connection Factory objects.
*MsgVpnApi* | [**get_msg_vpn_jndi_queues**](docs/MsgVpnApi.md#get_msg_vpn_jndi_queues) | **Get** /msgVpns/{msgVpnName}/jndiQueues | Gets a list of JNDI Queue objects.
*MsgVpnApi* | [**get_msg_vpn_jndi_topics**](docs/MsgVpnApi.md#get_msg_vpn_jndi_topics) | **Get** /msgVpns/{msgVpnName}/jndiTopics | Gets a list of JNDI Topic objects.
*MsgVpnApi* | [**get_msg_vpn_mqtt_sessions**](docs/MsgVpnApi.md#get_msg_vpn_mqtt_sessions) | **Get** /msgVpns/{msgVpnName}/mqttSessions | Gets a list of MQTT Session objects.
*MsgVpnApi* | [**get_msg_vpn_queues**](docs/MsgVpnApi.md#get_msg_vpn_queues) | **Get** /msgVpns/{msgVpnName}/queues | Gets a list of Queue objects.
*MsgVpnApi* | [**get_msg_vpn_replay_logs**](docs/MsgVpnApi.md#get_msg_vpn_replay_logs) | **Get** /msgVpns/{msgVpnName}/replayLogs | Gets a list of ReplayLog objects.
*MsgVpnApi* | [**get_msg_vpn_replicated_topics**](docs/MsgVpnApi.md#get_msg_vpn_replicated_topics) | **Get** /msgVpns/{msgVpnName}/replicatedTopics | Gets a list of Replicated Topic objects.
*MsgVpnApi* | [**get_msg_vpn_rest_delivery_points**](docs/MsgVpnApi.md#get_msg_vpn_rest_delivery_points) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints | Gets a list of REST Delivery Point objects.
*MsgVpnApi* | [**get_msg_vpn_topic_endpoints**](docs/MsgVpnApi.md#get_msg_vpn_topic_endpoints) | **Get** /msgVpns/{msgVpnName}/topicEndpoints | Gets a list of Topic Endpoint objects.
*MsgVpnApi* | [**get_msg_vpns**](docs/MsgVpnApi.md#get_msg_vpns) | **Get** /msgVpns | Gets a list of Message VPN objects.
*QueueApi* | [**get_msg_vpn_queues**](docs/QueueApi.md#get_msg_vpn_queues) | **Get** /msgVpns/{msgVpnName}/queues | Gets a list of Queue objects.
*ReplayLogApi* | [**get_msg_vpn_replay_logs**](docs/ReplayLogApi.md#get_msg_vpn_replay_logs) | **Get** /msgVpns/{msgVpnName}/replayLogs | Gets a list of ReplayLog objects.
*ReplicatedTopicApi* | [**get_msg_vpn_replicated_topics**](docs/ReplicatedTopicApi.md#get_msg_vpn_replicated_topics) | **Get** /msgVpns/{msgVpnName}/replicatedTopics | Gets a list of Replicated Topic objects.
*RestDeliveryPointApi* | [**get_msg_vpn_rest_delivery_points**](docs/RestDeliveryPointApi.md#get_msg_vpn_rest_delivery_points) | **Get** /msgVpns/{msgVpnName}/restDeliveryPoints | Gets a list of REST Delivery Point objects.
*SystemInformationApi* | [**get_system_information**](docs/SystemInformationApi.md#get_system_information) | **Get** /systemInformation | Gets SEMP API version and platform information.
*TopicEndpointApi* | [**get_msg_vpn_topic_endpoints**](docs/TopicEndpointApi.md#get_msg_vpn_topic_endpoints) | **Get** /msgVpns/{msgVpnName}/topicEndpoints | Gets a list of Topic Endpoint objects.


## Documentation For Models

 - [AboutApi](docs/AboutApi.md)
 - [AboutApiLinks](docs/AboutApiLinks.md)
 - [AboutApiResponse](docs/AboutApiResponse.md)
 - [AboutUser](docs/AboutUser.md)
 - [AboutUserLinks](docs/AboutUserLinks.md)
 - [AboutUserMsgVpn](docs/AboutUserMsgVpn.md)
 - [AboutUserMsgVpnLinks](docs/AboutUserMsgVpnLinks.md)
 - [AboutUserMsgVpnResponse](docs/AboutUserMsgVpnResponse.md)
 - [AboutUserMsgVpnsResponse](docs/AboutUserMsgVpnsResponse.md)
 - [AboutUserResponse](docs/AboutUserResponse.md)
 - [EventThreshold](docs/EventThreshold.md)
 - [EventThresholdByPercent](docs/EventThresholdByPercent.md)
 - [EventThresholdByValue](docs/EventThresholdByValue.md)
 - [MsgVpn](docs/MsgVpn.md)
 - [MsgVpnAclProfile](docs/MsgVpnAclProfile.md)
 - [MsgVpnAclProfileClientConnectException](docs/MsgVpnAclProfileClientConnectException.md)
 - [MsgVpnAclProfileClientConnectExceptionLinks](docs/MsgVpnAclProfileClientConnectExceptionLinks.md)
 - [MsgVpnAclProfileClientConnectExceptionResponse](docs/MsgVpnAclProfileClientConnectExceptionResponse.md)
 - [MsgVpnAclProfileClientConnectExceptionsResponse](docs/MsgVpnAclProfileClientConnectExceptionsResponse.md)
 - [MsgVpnAclProfileLinks](docs/MsgVpnAclProfileLinks.md)
 - [MsgVpnAclProfilePublishException](docs/MsgVpnAclProfilePublishException.md)
 - [MsgVpnAclProfilePublishExceptionLinks](docs/MsgVpnAclProfilePublishExceptionLinks.md)
 - [MsgVpnAclProfilePublishExceptionResponse](docs/MsgVpnAclProfilePublishExceptionResponse.md)
 - [MsgVpnAclProfilePublishExceptionsResponse](docs/MsgVpnAclProfilePublishExceptionsResponse.md)
 - [MsgVpnAclProfileResponse](docs/MsgVpnAclProfileResponse.md)
 - [MsgVpnAclProfileSubscribeException](docs/MsgVpnAclProfileSubscribeException.md)
 - [MsgVpnAclProfileSubscribeExceptionLinks](docs/MsgVpnAclProfileSubscribeExceptionLinks.md)
 - [MsgVpnAclProfileSubscribeExceptionResponse](docs/MsgVpnAclProfileSubscribeExceptionResponse.md)
 - [MsgVpnAclProfileSubscribeExceptionsResponse](docs/MsgVpnAclProfileSubscribeExceptionsResponse.md)
 - [MsgVpnAclProfilesResponse](docs/MsgVpnAclProfilesResponse.md)
 - [MsgVpnAuthorizationGroup](docs/MsgVpnAuthorizationGroup.md)
 - [MsgVpnAuthorizationGroupLinks](docs/MsgVpnAuthorizationGroupLinks.md)
 - [MsgVpnAuthorizationGroupResponse](docs/MsgVpnAuthorizationGroupResponse.md)
 - [MsgVpnAuthorizationGroupsResponse](docs/MsgVpnAuthorizationGroupsResponse.md)
 - [MsgVpnBridge](docs/MsgVpnBridge.md)
 - [MsgVpnBridgeLinks](docs/MsgVpnBridgeLinks.md)
 - [MsgVpnBridgeRemoteMsgVpn](docs/MsgVpnBridgeRemoteMsgVpn.md)
 - [MsgVpnBridgeRemoteMsgVpnLinks](docs/MsgVpnBridgeRemoteMsgVpnLinks.md)
 - [MsgVpnBridgeRemoteMsgVpnResponse](docs/MsgVpnBridgeRemoteMsgVpnResponse.md)
 - [MsgVpnBridgeRemoteMsgVpnsResponse](docs/MsgVpnBridgeRemoteMsgVpnsResponse.md)
 - [MsgVpnBridgeRemoteSubscription](docs/MsgVpnBridgeRemoteSubscription.md)
 - [MsgVpnBridgeRemoteSubscriptionLinks](docs/MsgVpnBridgeRemoteSubscriptionLinks.md)
 - [MsgVpnBridgeRemoteSubscriptionResponse](docs/MsgVpnBridgeRemoteSubscriptionResponse.md)
 - [MsgVpnBridgeRemoteSubscriptionsResponse](docs/MsgVpnBridgeRemoteSubscriptionsResponse.md)
 - [MsgVpnBridgeResponse](docs/MsgVpnBridgeResponse.md)
 - [MsgVpnBridgeTlsTrustedCommonName](docs/MsgVpnBridgeTlsTrustedCommonName.md)
 - [MsgVpnBridgeTlsTrustedCommonNameLinks](docs/MsgVpnBridgeTlsTrustedCommonNameLinks.md)
 - [MsgVpnBridgeTlsTrustedCommonNameResponse](docs/MsgVpnBridgeTlsTrustedCommonNameResponse.md)
 - [MsgVpnBridgeTlsTrustedCommonNamesResponse](docs/MsgVpnBridgeTlsTrustedCommonNamesResponse.md)
 - [MsgVpnBridgesResponse](docs/MsgVpnBridgesResponse.md)
 - [MsgVpnClientProfile](docs/MsgVpnClientProfile.md)
 - [MsgVpnClientProfileLinks](docs/MsgVpnClientProfileLinks.md)
 - [MsgVpnClientProfileResponse](docs/MsgVpnClientProfileResponse.md)
 - [MsgVpnClientProfilesResponse](docs/MsgVpnClientProfilesResponse.md)
 - [MsgVpnClientUsername](docs/MsgVpnClientUsername.md)
 - [MsgVpnClientUsernameLinks](docs/MsgVpnClientUsernameLinks.md)
 - [MsgVpnClientUsernameResponse](docs/MsgVpnClientUsernameResponse.md)
 - [MsgVpnClientUsernamesResponse](docs/MsgVpnClientUsernamesResponse.md)
 - [MsgVpnJndiConnectionFactoriesResponse](docs/MsgVpnJndiConnectionFactoriesResponse.md)
 - [MsgVpnJndiConnectionFactory](docs/MsgVpnJndiConnectionFactory.md)
 - [MsgVpnJndiConnectionFactoryLinks](docs/MsgVpnJndiConnectionFactoryLinks.md)
 - [MsgVpnJndiConnectionFactoryResponse](docs/MsgVpnJndiConnectionFactoryResponse.md)
 - [MsgVpnJndiQueue](docs/MsgVpnJndiQueue.md)
 - [MsgVpnJndiQueueLinks](docs/MsgVpnJndiQueueLinks.md)
 - [MsgVpnJndiQueueResponse](docs/MsgVpnJndiQueueResponse.md)
 - [MsgVpnJndiQueuesResponse](docs/MsgVpnJndiQueuesResponse.md)
 - [MsgVpnJndiTopic](docs/MsgVpnJndiTopic.md)
 - [MsgVpnJndiTopicLinks](docs/MsgVpnJndiTopicLinks.md)
 - [MsgVpnJndiTopicResponse](docs/MsgVpnJndiTopicResponse.md)
 - [MsgVpnJndiTopicsResponse](docs/MsgVpnJndiTopicsResponse.md)
 - [MsgVpnLinks](docs/MsgVpnLinks.md)
 - [MsgVpnMqttSession](docs/MsgVpnMqttSession.md)
 - [MsgVpnMqttSessionLinks](docs/MsgVpnMqttSessionLinks.md)
 - [MsgVpnMqttSessionResponse](docs/MsgVpnMqttSessionResponse.md)
 - [MsgVpnMqttSessionSubscription](docs/MsgVpnMqttSessionSubscription.md)
 - [MsgVpnMqttSessionSubscriptionLinks](docs/MsgVpnMqttSessionSubscriptionLinks.md)
 - [MsgVpnMqttSessionSubscriptionResponse](docs/MsgVpnMqttSessionSubscriptionResponse.md)
 - [MsgVpnMqttSessionSubscriptionsResponse](docs/MsgVpnMqttSessionSubscriptionsResponse.md)
 - [MsgVpnMqttSessionsResponse](docs/MsgVpnMqttSessionsResponse.md)
 - [MsgVpnQueue](docs/MsgVpnQueue.md)
 - [MsgVpnQueueLinks](docs/MsgVpnQueueLinks.md)
 - [MsgVpnQueueResponse](docs/MsgVpnQueueResponse.md)
 - [MsgVpnQueueSubscription](docs/MsgVpnQueueSubscription.md)
 - [MsgVpnQueueSubscriptionLinks](docs/MsgVpnQueueSubscriptionLinks.md)
 - [MsgVpnQueueSubscriptionResponse](docs/MsgVpnQueueSubscriptionResponse.md)
 - [MsgVpnQueueSubscriptionsResponse](docs/MsgVpnQueueSubscriptionsResponse.md)
 - [MsgVpnQueuesResponse](docs/MsgVpnQueuesResponse.md)
 - [MsgVpnReplayLog](docs/MsgVpnReplayLog.md)
 - [MsgVpnReplayLogLinks](docs/MsgVpnReplayLogLinks.md)
 - [MsgVpnReplayLogResponse](docs/MsgVpnReplayLogResponse.md)
 - [MsgVpnReplayLogsResponse](docs/MsgVpnReplayLogsResponse.md)
 - [MsgVpnReplicatedTopic](docs/MsgVpnReplicatedTopic.md)
 - [MsgVpnReplicatedTopicLinks](docs/MsgVpnReplicatedTopicLinks.md)
 - [MsgVpnReplicatedTopicResponse](docs/MsgVpnReplicatedTopicResponse.md)
 - [MsgVpnReplicatedTopicsResponse](docs/MsgVpnReplicatedTopicsResponse.md)
 - [MsgVpnResponse](docs/MsgVpnResponse.md)
 - [MsgVpnRestDeliveryPoint](docs/MsgVpnRestDeliveryPoint.md)
 - [MsgVpnRestDeliveryPointLinks](docs/MsgVpnRestDeliveryPointLinks.md)
 - [MsgVpnRestDeliveryPointQueueBinding](docs/MsgVpnRestDeliveryPointQueueBinding.md)
 - [MsgVpnRestDeliveryPointQueueBindingLinks](docs/MsgVpnRestDeliveryPointQueueBindingLinks.md)
 - [MsgVpnRestDeliveryPointQueueBindingResponse](docs/MsgVpnRestDeliveryPointQueueBindingResponse.md)
 - [MsgVpnRestDeliveryPointQueueBindingsResponse](docs/MsgVpnRestDeliveryPointQueueBindingsResponse.md)
 - [MsgVpnRestDeliveryPointResponse](docs/MsgVpnRestDeliveryPointResponse.md)
 - [MsgVpnRestDeliveryPointRestConsumer](docs/MsgVpnRestDeliveryPointRestConsumer.md)
 - [MsgVpnRestDeliveryPointRestConsumerLinks](docs/MsgVpnRestDeliveryPointRestConsumerLinks.md)
 - [MsgVpnRestDeliveryPointRestConsumerResponse](docs/MsgVpnRestDeliveryPointRestConsumerResponse.md)
 - [MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonName](docs/MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonName.md)
 - [MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameLinks](docs/MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameLinks.md)
 - [MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse](docs/MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNameResponse.md)
 - [MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNamesResponse](docs/MsgVpnRestDeliveryPointRestConsumerTlsTrustedCommonNamesResponse.md)
 - [MsgVpnRestDeliveryPointRestConsumersResponse](docs/MsgVpnRestDeliveryPointRestConsumersResponse.md)
 - [MsgVpnRestDeliveryPointsResponse](docs/MsgVpnRestDeliveryPointsResponse.md)
 - [MsgVpnSequencedTopic](docs/MsgVpnSequencedTopic.md)
 - [MsgVpnSequencedTopicLinks](docs/MsgVpnSequencedTopicLinks.md)
 - [MsgVpnSequencedTopicResponse](docs/MsgVpnSequencedTopicResponse.md)
 - [MsgVpnSequencedTopicsResponse](docs/MsgVpnSequencedTopicsResponse.md)
 - [MsgVpnTopicEndpoint](docs/MsgVpnTopicEndpoint.md)
 - [MsgVpnTopicEndpointLinks](docs/MsgVpnTopicEndpointLinks.md)
 - [MsgVpnTopicEndpointResponse](docs/MsgVpnTopicEndpointResponse.md)
 - [MsgVpnTopicEndpointsResponse](docs/MsgVpnTopicEndpointsResponse.md)
 - [MsgVpnsResponse](docs/MsgVpnsResponse.md)
 - [SempError](docs/SempError.md)
 - [SempMeta](docs/SempMeta.md)
 - [SempMetaOnlyResponse](docs/SempMetaOnlyResponse.md)
 - [SempPaging](docs/SempPaging.md)
 - [SempRequest](docs/SempRequest.md)
 - [SystemInformation](docs/SystemInformation.md)
 - [SystemInformationLinks](docs/SystemInformationLinks.md)
 - [SystemInformationResponse](docs/SystemInformationResponse.md)


## Documentation For Authorization

## basicAuth
- **Type**: HTTP basic authentication

Example
```
	auth := context.WithValue(context.TODO(), sw.ContextBasicAuth, sw.BasicAuth{
		UserName: "username",
		Password: "password",
	})
    r, err := client.Service.Operation(auth, args)
```

## Author

support@solace.com

