import JsonView from "@uiw/react-json-view";
import { useEffect, useState } from "react";
import { PageContainer } from "@toolpad/core/PageContainer";
import { Container } from "@mui/material";
import Queues from "./components/Queues";
import Vhosts from "./components/Vhosts";

export default function App() {
  const [vhosts, setVhosts] = useState([]);
  const [selectedVhost, setSelectedVhost] = useState("");
  const [queues, setQueues] = useState([]);
  const [selectedQueue, setSelectedQueue] = useState("");

  useEffect(() => {
    async function responses() {
      let response = await fetch("/vhosts");
      setVhosts(await response.json());
    }
    responses();
  }, []);

  /**
   * Handles the change event for the virtual host selection.
   *
   * This function is called when the user selects a different virtual host from the dropdown menu.
   * It updates the state with the selected virtual host.
   *
   * @param {Object} event - The event object from the dropdown menu.
   */
  const handleChange = async (event) => {
    const vhost = event.target.value;
    setSelectedVhost(vhost);
    await getQueues(vhost);
  };

  /**
   * Fetches the list of queues for a given virtual host.
   *
   * This asynchronous function sends an HTTP GET request to the server to retrieve the list of queues
   * for the specified virtual host. The response is then parsed as JSON and the state is updated with the list of queues.
   *
   * @param {string} vhost - The name of the virtual host for which to retrieve the queues.
   */
  const getQueues = async (vhost) => {
    let response = await fetch(`/queues/${vhost}`);
    setQueues(await response.json());
  };

  /**
   * Renders a JSON view inside a container.
   *
   * This function takes a JSON object as input and returns a JSX element that displays
   * the JSON object using the `JsonView` component inside a `Container` component.
   *
   * @param {Object} values - The JSON object to be displayed.
   * @returns {JSX.Element} A JSX element containing the JSON view.
   */
  const renderJson = (values) => {
    return (
      <Container>
        <JsonView value={values} />
      </Container>
    );
  };

  return (
    <>
      <PageContainer>
        <Vhosts
          selectedVhost={selectedVhost}
          handleChange={handleChange}
          vhosts={vhosts}
        />
        {queues && <Queues queues={queues} />}
      </PageContainer>
    </>
  );
}
