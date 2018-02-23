import React, { Component } from 'react'

import { Row } from 'react-bootstrap';

export default class CSV extends Component {
    render() {
        const rows = () => {
          return this.props.data.map((entry, index) => {
              return <div key={index}>
                {`"${entry.product}",${entry.price},${entry.time},"${entry.category}"`}
              <br/>
              </div>
          })
        }
        return <Row>
            {rows()}
          </Row>;
    }
}
