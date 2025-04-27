import TitleSearchField from "./client/components/TitleSearchField"
import Box from '@mui/material/Box'
import Container from '@mui/material/Container'

function App() {
  return (
    <>
      <Container maxWidth="xl" sx={{ minHeight: '80vh', py: 2 }}>
        <h1>Movie Search</h1>
        <Box sx={{ width: '100%', maxWidth: 400, margin: '0 auto' }}>
          <TitleSearchField value={"test"} />
        </Box>
      </Container>
    </>
  )
}

export default App
