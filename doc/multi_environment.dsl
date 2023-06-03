workspace {

  model {
    user = person "User"
    commandLineClient = softwareSystem "Command Line Client"
    webClient = softwareSystem "Web Client"
    desktopClient = softwareSystem "Desktop Client"
    environment = softwareSystem "Environment" "CoreFlow services instance"
    
    user -> commandLineClient "Interacts with via command line"
    user -> webClient "Interacts with via web browser"
    user -> desktopClient "Interacts with via desktop application"
    
    commandLineClient -> environment "Connects to single environment per invocation"
    webClient -> environment "Connects to single environment"
    desktopClient -> environment "Connects to multiple environments concurrently"
  }

  views {
    systemContext environment {
      include *
      autoLayout
    }

    styles {
      element "Software System" {
        background #1168bd
        color #ffffff
      }
      element "Person" {
        background #08427b
        color #ffffff
      }
    }
  }
}
