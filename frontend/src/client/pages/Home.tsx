import App from '../components/App'
import Layout from '../components/Layout';
import usePageTracking from '../hooks/useTracking.ts'

const Home = () => {
  usePageTracking();

  return (
    <Layout>
      <App />
    </Layout>
  )
}

export default Home;