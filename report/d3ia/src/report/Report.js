import React, { Component } from 'react'

import { Tab, Row, Col, Nav, NavItem } from 'react-bootstrap';

import Main from './Main';
import Entries from './Entries';
import CSV from './CSV';

export default class Report extends Component {
    render() {
        return <Tab.Container id="left-tabs-example" defaultActiveKey="first">
            <Row className="clearfix">
                <Col md={2} sm={2}>
                  <Nav bsStyle="pills" stacked>
                    <NavItem eventKey="first">Отчет</NavItem>
                    <NavItem eventKey="second">Данные</NavItem>
                    <NavItem eventKey="third">CSV</NavItem>
                  </Nav>
                </Col>
                <Col md={10} sm={10}>
                  <Tab.Content animation>
                    <Tab.Pane eventKey="first">
                        {this.props.data.title}({this.props.data.timePeriod.from}-{this.props.data.timePeriod.to})
                        <Main data={this.props.data.main}/>
                    </Tab.Pane>
                    <Tab.Pane eventKey="second">
                        <Entries data={this.props.data.entries}/>
                    </Tab.Pane>
                    <Tab.Pane eventKey="third">
                        <CSV data={this.props.data.entries}/>
                    </Tab.Pane>
                  </Tab.Content>
                </Col>
            </Row>
    </Tab.Container>;
    }
}
