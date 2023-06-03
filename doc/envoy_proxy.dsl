workspace {

  model {
    user = person "User"
    environment = group "Environment" {
        envoyProxy = softwareSystem "Envoy Proxy" "Serves as entry point for all network traffic"
        serviceA = softwareSystem "Service A" "Provides feature A" "Rust"
        serviceB = softwareSystem "Service B" "Provides feature B" "Rust"
        serviceC = softwareSystem "Service C" "Provides feature C" "Rust"
    }
    
    user -> envoyProxy "Sends requests to"
    envoyProxy -> serviceA "Routes requests to"
    envoyProxy -> serviceB "Routes requests to"
    envoyProxy -> serviceC "Routes requests to"
  }

  views {
    styles {
      element "Software System" {
        background #1168bd
        color #ffffff
      }
      element "Person" {
        background #08427b
        color #ffffff
      }
      element "Container" {
        background #28a745
        color #ffffff
      }
    }
  }
}
