import { useEffect, useState } from "react";
import { PageContainer } from "@toolpad/core/PageContainer";
import Queues from "./components/Queues";
import Vhosts from "./components/Vhosts";
import { Routes, Route, useNavigate } from "react-router-dom";
import Queue from "./components/Queue";
import { Box } from "@mui/material";
import AppBar from "@mui/material/AppBar";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import { Link } from "react-router-dom";
import { styled } from "@mui/material/styles";
import MuiCard from "@mui/material/Card";

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

  const Card = styled(MuiCard)(({ theme }) => ({
    display: "flex",
    flexDirection: "column",
    alignSelf: "center",
    width: "100%",
    padding: theme.spacing(4),
    gap: theme.spacing(2),
    margin: "auto",
    [theme.breakpoints.up("sm")]: {
      maxWidth: "450px",
    },
    boxShadow:
      "hsla(220, 30%, 5%, 0.05) 0px 5px 15px 0px, hsla(220, 25%, 10%, 0.05) 0px 15px 35px -5px, hsla(220, 20%, 15%, 0.1) 0px 25px 45px -10px, hsla(220, 15%, 20%, 0.15) 0px 35px 55px -15px",
    ...theme.applyStyles("dark", {
      boxShadow:
        "hsla(220, 30%, 5%, 0.5) 0px 5px 15px 0px, hsla(220, 25%, 10%, 0.08) 0px 15px 35px -5px, hsla(220, 20%, 15%, 0.12) 0px 25px 45px -10px, hsla(220, 15%, 20%, 0.18) 0px 35px 55px -15px",
    }),
  }));

  return (
    <>
      <Box
        sx={{
          background:
            "linear-gradient(to bottom right, orange, #f07926, #ff9600, #ffb74d, white)",
        }}
      >
        <Box>
          <AppBar position="static" color="warning">
            <Toolbar>
              <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
                <Link
                  to="/"
                  style={{ textDecoration: "none", color: "inherit" }}
                >
                  RabbitMQ Message Viewer
                </Link>
              </Typography>
            </Toolbar>
          </AppBar>
        </Box>

        <PageContainer>
          <Box
            display="flex"
            flexDirection="column"
            justifyContent="center"
            height="95vh"
          >
            <Routes>
              <Route
                path="/"
                element={
                  <>
                    <Card variant="outlined">
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
                    </Card>
                  </>
                }
              />
              <Route path=":vhost/queues/:queue" element={<Queue />} />
            </Routes>
          </Box>
        </PageContainer>
      </Box>
    </>
  );
}
