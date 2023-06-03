workspace {

    model {
        user = person "User"
        datablock = softwareSystem "DataBlock"
        jobtrigger = softwareSystem "JobTrigger"
        job = softwareSystem "Job"
        jobblock = softwareSystem "JobBlock"
        jobqueue = softwareSystem "JobQueue"
        worker = softwareSystem "Worker"
        scheduler = softwareSystem "Scheduler"
        scaler = softwareSystem "Scaler"

        user -> jobtrigger "Triggers"
        jobtrigger -> job "Queues"
        jobtrigger -> datablock "Depends on"
        job -> datablock "Outputs"
        job -> jobblock "Is part of"
        jobqueue -> job "Contains"
        worker -> job "Executes"
        scheduler -> jobqueue "Checks"
        scheduler -> worker "Assigns Jobs"
        scaler -> jobqueue "Checks"
        scaler -> worker "Creates or Destroys"
    }

    views {
        systemLandscape {
            include *
            autoLayout
        }
    }
}