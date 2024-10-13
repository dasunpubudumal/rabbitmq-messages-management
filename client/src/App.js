import { useEffect, useState } from "react";
import { PageContainer } from "@toolpad/core/PageContainer";
import Queues from "./components/Queues";
import Vhosts from "./components/Vhosts";
import { Routes, Route, useNavigate } from "react-router-dom";
import Queue from "./components/Queue";

export default function App() {
  const [vhosts, setVhosts] = useState([]);
  const [selectedVhost, setSelectedVhost] = useState("");
  const [queues, setQueues] = useState([]);
  const [selectedQueue, setSelectedQueue] = useState("");
  const navigate = useNavigate();

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
  const handleVhostChange = async (event) => {
    const vhost = event.target.value;
    setSelectedVhost(vhost);
    await getQueues(vhost);
  };

  /**
   * Handles the change event for the queue selection.
   *
   * This function is called when the user selects a different queue from the dropdown menu.
   * It updates the state with the selected queue.
   *
   * @param {Object} event - The event object from the dropdown menu.
   */
  const handleSelectedQueueChange = (queueName) => {
    setSelectedQueue(queueName);
    console.log(`Selected queue: ${queueName}`);
    navigate(`${selectedVhost}/queues/${queueName}`);
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

  return (
    <>
      <PageContainer>
        <Routes>
          <Route
            path="/"
            element={
              <>
                <Vhosts
                  selectedVhost={selectedVhost}
                  handleChange={handleVhostChange}
                  vhosts={vhosts}
                />
                {queues && (
                  <Queues
                    queues={queues}
                    handleSelectedQueueChange={handleSelectedQueueChange}
                  />
                )}
              </>
            }
          />
          <Route path=":vhost/queues/:queue" element={<Queue />} />
        </Routes>
      </PageContainer>
    </>
  );
}
