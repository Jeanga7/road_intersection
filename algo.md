# Traffic Management Algorithm

## Overview
The algorithm manages traffic at a four-way intersection with traffic lights. It handles vehicle movement, traffic light changes, and determines which lane gets priority to cross the intersection.

## Key Components
1. Lanes: Four lanes (First, Second, Third, Fourth) representing each direction of the intersection.
2. Vehicles: Cars moving through the lanes with different routes (Up, Down, Left, Right).
3. Traffic Lights: Each lane has a traffic light (Red or Green).
4. Stages: Waiting (vehicles stop at red light) and Crossing (vehicles can move through intersection).

## Detailed Algorithm

### 1. Vehicle Management
a. Vehicle Creation and Spawning:
   - Vehicles are created with a specific route and color.
   - They spawn at predefined points based on their route.

b. Vehicle Movement:
   - Vehicles move forward in their lane.
   - They stop if they reach the stop point and the traffic light is red.
   - They change direction based on their color and destination.

c. Vehicle Removal:
   - Vehicles are removed when they reach the end of their route.

### 2. Traffic Light Management
a. Traffic Light States:
   - Each lane has a traffic light that can be either Red or Green.

b. Changing Traffic Lights:
   - Traffic lights change based on the current traffic situation.

### 3. Lane Management
a. Stopping Vehicles:
   - Vehicles stop at the designated stop point if the traffic light is red.
   - Vehicles maintain a safety distance from the vehicle in front.

b. Crossing Stage:
   - When a lane enters the Crossing stage, vehicles can move through the intersection.

### 4. Intersection Management
a. Determining Next Lane to Cross:
   1. Check if any lane is currently in the Crossing stage. If so, no change is made.
   2. If no lane is crossing, evaluate each lane:
      - Filter for waiting vehicles in each lane.
      - Calculate the closest vehicle's distance to the stop point.
      - Count the number of waiting vehicles.
   3. Select the lane with:
      - The closest vehicle to the stop point, OR
      - If distances are equal, the lane with the most waiting vehicles.

b. Changing Intersection State:
   - Set the selected lane's traffic light to Green.
   - Change the selected lane's stage to Crossing.

### 5. Main Loop
1. Handle user input (adding new vehicles).
2. Update each lane:
   - Move vehicles
   - Update traffic lights
   - Remove vehicles that have completed their route
3. Update traffic lights based on the intersection management algorithm.
4. Render the updated state.
5. Repeat from step 1.

## Key Functions
- `update_traffic_lights()`: Implements the intersection management algorithm.
- `Lane::update()`: Updates the state of vehicles in a lane.
- `Lane::stop_vehicles()`: Stops vehicles at red lights and maintains safety distances.
- `Lane::cross()`: Manages the crossing of vehicles through the intersection.
- `Vehicle::move_forward()`: Handles the movement and direction changes of individual vehicles.

This algorithm balances traffic flow by prioritizing lanes with vehicles closest to the intersection and considering the number of waiting vehicles to prevent long queues in any single direction.