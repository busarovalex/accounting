import React, { Component } from 'react'

import { Row } from 'react-bootstrap';

import { PieChart } from 'react-d3-basic';

export default class Main extends Component {
    render() {

        const chartSeries = this.props.data.map((entry) => {
          return {
            field: entry.category,
            name: `${entry.category} ${entry.total} руб (${entry.persent}%)`
          }
        })

        const data = this.props.data;

        const value = (d) => {
          return d.total;
        }

        const name = (d) => {
          return d.category
        }

        return <Row>

            <PieChart width={950} height={500} 
                      chartSeries= {chartSeries}
                      data={data} 
                      name={name}
                      value={value}/>
          </Row>;
    }
}
