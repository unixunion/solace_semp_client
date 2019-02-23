use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  about_api: Box<::apis::AboutApi>,
  acl_profile_api: Box<::apis::AclProfileApi>,
  authorization_group_api: Box<::apis::AuthorizationGroupApi>,
  bridge_api: Box<::apis::BridgeApi>,
  client_profile_api: Box<::apis::ClientProfileApi>,
  client_username_api: Box<::apis::ClientUsernameApi>,
  default_api: Box<::apis::DefaultApi>,
  jndi_api: Box<::apis::JndiApi>,
  mqtt_session_api: Box<::apis::MqttSessionApi>,
  msg_vpn_api: Box<::apis::MsgVpnApi>,
  queue_api: Box<::apis::QueueApi>,
  replay_log_api: Box<::apis::ReplayLogApi>,
  replicated_topic_api: Box<::apis::ReplicatedTopicApi>,
  rest_delivery_point_api: Box<::apis::RestDeliveryPointApi>,
  system_information_api: Box<::apis::SystemInformationApi>,
  topic_endpoint_api: Box<::apis::TopicEndpointApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      about_api: Box::new(::apis::AboutApiClient::new(rc.clone())),
      acl_profile_api: Box::new(::apis::AclProfileApiClient::new(rc.clone())),
      authorization_group_api: Box::new(::apis::AuthorizationGroupApiClient::new(rc.clone())),
      bridge_api: Box::new(::apis::BridgeApiClient::new(rc.clone())),
      client_profile_api: Box::new(::apis::ClientProfileApiClient::new(rc.clone())),
      client_username_api: Box::new(::apis::ClientUsernameApiClient::new(rc.clone())),
      default_api: Box::new(::apis::DefaultApiClient::new(rc.clone())),
      jndi_api: Box::new(::apis::JndiApiClient::new(rc.clone())),
      mqtt_session_api: Box::new(::apis::MqttSessionApiClient::new(rc.clone())),
      msg_vpn_api: Box::new(::apis::MsgVpnApiClient::new(rc.clone())),
      queue_api: Box::new(::apis::QueueApiClient::new(rc.clone())),
      replay_log_api: Box::new(::apis::ReplayLogApiClient::new(rc.clone())),
      replicated_topic_api: Box::new(::apis::ReplicatedTopicApiClient::new(rc.clone())),
      rest_delivery_point_api: Box::new(::apis::RestDeliveryPointApiClient::new(rc.clone())),
      system_information_api: Box::new(::apis::SystemInformationApiClient::new(rc.clone())),
      topic_endpoint_api: Box::new(::apis::TopicEndpointApiClient::new(rc.clone())),
    }
  }

  pub fn about_api(&self) -> &::apis::AboutApi{
    self.about_api.as_ref()
  }

  pub fn acl_profile_api(&self) -> &::apis::AclProfileApi{
    self.acl_profile_api.as_ref()
  }

  pub fn authorization_group_api(&self) -> &::apis::AuthorizationGroupApi{
    self.authorization_group_api.as_ref()
  }

  pub fn bridge_api(&self) -> &::apis::BridgeApi{
    self.bridge_api.as_ref()
  }

  pub fn client_profile_api(&self) -> &::apis::ClientProfileApi{
    self.client_profile_api.as_ref()
  }

  pub fn client_username_api(&self) -> &::apis::ClientUsernameApi{
    self.client_username_api.as_ref()
  }

  pub fn default_api(&self) -> &::apis::DefaultApi{
    self.default_api.as_ref()
  }

  pub fn jndi_api(&self) -> &::apis::JndiApi{
    self.jndi_api.as_ref()
  }

  pub fn mqtt_session_api(&self) -> &::apis::MqttSessionApi{
    self.mqtt_session_api.as_ref()
  }

  pub fn msg_vpn_api(&self) -> &::apis::MsgVpnApi{
    self.msg_vpn_api.as_ref()
  }

  pub fn queue_api(&self) -> &::apis::QueueApi{
    self.queue_api.as_ref()
  }

  pub fn replay_log_api(&self) -> &::apis::ReplayLogApi{
    self.replay_log_api.as_ref()
  }

  pub fn replicated_topic_api(&self) -> &::apis::ReplicatedTopicApi{
    self.replicated_topic_api.as_ref()
  }

  pub fn rest_delivery_point_api(&self) -> &::apis::RestDeliveryPointApi{
    self.rest_delivery_point_api.as_ref()
  }

  pub fn system_information_api(&self) -> &::apis::SystemInformationApi{
    self.system_information_api.as_ref()
  }

  pub fn topic_endpoint_api(&self) -> &::apis::TopicEndpointApi{
    self.topic_endpoint_api.as_ref()
  }


}
