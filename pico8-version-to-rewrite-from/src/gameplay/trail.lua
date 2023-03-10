-- -- -- -- -- -- --
-- gameplay/trail --
-- -- -- -- -- -- --

function new_trail(params)
    local origin = params.origin
    local color = params.color

    local frames_between_particles = 4
    local frame_counter = frames_between_particles

    local particles = {}

    local t = {}

    --

    function t.update()
        for particle in all(particles) do
            particle.age()
        end
        for i = 1, #particles do
            if particles[i] then
                particles[i].age()
                if particles[i].should_disappear() then
                    deli(particles, i)
                    particles[i] = particles[#particles]
                    particles[#particles] = nil
                end
            end
        end

        if frame_counter <= 0 then
            add(particles, new_particle {
                x = origin.xc(),
                y = origin.yc(),
                -- TODO
                --color = color,
            })
        end

        frame_counter = (frame_counter + 1) % frames_between_particles
    end

    --

    function t.draw()
        for particle in all(particles) do
            particle.draw()
        end
    end

    --

    return t
end