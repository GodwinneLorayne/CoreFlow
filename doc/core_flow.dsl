workspace {

  model {
    user = person "User" "A user of the CoreFlow system"
    coreFlow = softwareSystem "CoreFlow" "The CI/CD system" {
        clientService = container "Client Service" "Handles job definition, dependency specification, and retrieval of build results and logs" "Rust"
        schedulerService = container "Scheduler Service" "Handles job scheduling and manages fork-join parallelism" "Rust"
        workerService = container "Worker Service" "Executes the jobs and returns the output and completion status" "Rust"
        storageService = container "Storage Service" "Manages data persistence" "Rust"
    
        user -> clientService "Submits jobs and retrieves results"
        clientService -> schedulerService "Submits job definitions"
        schedulerService -> workerService "Assigns tasks"
        workerService -> schedulerService "Returns completion status and output"
        workerService -> storageService "Stores build artifacts, logs, and output data"
        clientService -> storageService "Retrieves build results and logs"
    }

  }

  views {
    container coreFlow {
      include *
      autoLayout
    }

    styles {
      element "Software System" {
        background #1168bd
        color #ffffff
      }
      element "Container" {
        background #438dd5
        color #ffffff
      }
      element "Person" {
        background #08427b
        color #ffffff
      }
    }
  }
}
