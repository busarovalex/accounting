import React, { Component } from 'react';
import './App.css';
import Report from './report/Report';

import { Tabs, Tab, Grid, Row, Col } from 'react-bootstrap';

class App extends Component {

  constructor(props) {
    super(props);
    this.state = {
      data: [
        {
          title: "Всего",
          timePeriod: {from: "01.02.2018", to: "31.03.2018"},
          main:[
            {
              "category": "food",
              "total": 10,
              "persent": 10
            },
            {
              "category": "transport",
              "total": 90,
              "persent": 90
            }
          ],
          entries: [
            {"product": "coffee", "price": 5, "time": "2018-02-16T11:10:00.000000000", "category": "food"},
            {"product": "butter", "price": 5, "time": "2018-02-16T11:12:00.000000000", "category": "food"},
            {"product": "transport-card", "price": 5, "time": "2018-02-18T11:12:00.000000000", "category": "transport"},
          ]
        },
        {
          title: "Февраль",
          timePeriod: {from: "01.02.2018", to: "28.02.2018"},
          main:[
            {
              "category": "food",
              "total": 5,
              "persent": 100
            }
          ],
          entries: [
            {"product": "coffee", "price": 5, "time": "2018-02-16T11:10:00.000000000", "category": "food"}
          ]
        },
        {
          title: "Март",
          timePeriod: {from: "01.03.2018", to: "31.03.2018"},
          main:[
            {
              "category": "food",
              "total": 5,
              "persent": 7
            },
            {
              "category": "transport",
              "total": 90,
              "persent": 93
            }
          ],
          entries: [
            {"product": "coffee", "price": 5, "time": "2018-02-16T11:10:00.000000000", "category": "food"},
            {"product": "transport-card", "price": 5, "time": "2018-02-18T11:12:00.000000000", "category": "transport"},
          ]
        }
      ]
    }
  }

  render() {

    const tabs = () => this.state.data.map((entry, index) => <Tab key={index} title={entry.title} eventKey={index}><Report data={entry}/></Tab>)
    

    return (
      <Grid fluid={true}>
        <Row className="show-grid">
          <Col md={12}>
            <Tabs id={"topTabs"}>
              {tabs()}
            </Tabs>
          </Col>
        </Row>
      </Grid>
    );
  }
}

export default App;
