use yew::prelude::*;

struct Model {
    link: dyn Component,
    job_status: String,
}

enum Msg {
    GetJobStatus,
    ReceiveJobStatus(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            job_status: "Job Status: Unknown".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetJobStatus => {
                // Here, normally you'd make an async request to your CoreFlow server
                // using the Fetch API or WebSocket API to get job status.
                // We'll just simulate that with a simple string for this example.
                self.link
                    .send_message(Msg::ReceiveJobStatus("Job is running".into()));
                true
            }
            Msg::ReceiveJobStatus(status) => {
                self.job_status = format!("Job Status: {}", status);
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{ "CoreFlow" }</h1>
                <button onclick=self.link.callback(|_| Msg::GetJobStatus)>{ "Get Job Status" }</button>
                <p>{ &self.job_status }</p>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
