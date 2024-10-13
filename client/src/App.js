import JsonView from "@uiw/react-json-view";
import { useEffect, useState } from "react";
import InputLabel from "@mui/material/InputLabel";
import MenuItem from "@mui/material/MenuItem";
import FormControl from "@mui/material/FormControl";
import Select from "@mui/material/Select";
import { PageContainer } from '@toolpad/core/PageContainer';
import { Container } from "@mui/material";

export default function App() {
  const [vhosts, setVhosts] = useState([]);
  const [selectedVhost, setSelectedVhost] = useState("");

  useEffect(() => {
    async function responses() {
      let response = await fetch("/vhosts");
      setVhosts(await response.json());
    }
    responses();
  }, []);

  const handleChange = (event) => {
    setSelectedVhost(event.target.value);
  };

  return (
    <>
      <PageContainer>
        <FormControl fullWidth>
          <InputLabel id="vhost-select">Select Vhost</InputLabel>
          <Select
            labelId="vhost-select"
            id="vhost-select"
            value={selectedVhost}
            label="Age"
            onChange={handleChange}
          >
            {vhosts.map((vhost, index) => (
              <MenuItem value={vhost.name} key={index}>
                {vhost.name}
              </MenuItem>
            ))}
          </Select>
        </FormControl>
        <Container>
          <JsonView value={vhosts} />
        </Container>
      </PageContainer>
    </>
  );
}
