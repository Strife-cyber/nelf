import React from 'react';

interface ServiceDetailsProps {
  serviceId: string;
}

const ServiceDetails: React.FC<ServiceDetailsProps> = ({ serviceId }) => {
  const serviceDetails = {
    dev: {
      title: "Web & Mobile Development",
      content: (
        <div className="space-y-6">
          <p className="text-lg leading-relaxed text-gray-300">
            We architect and build cutting-edge digital solutions that push the boundaries of what's possible on the web and mobile platforms.
          </p>
          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-white">Our Expertise</h3>
            <ul className="space-y-2 text-gray-300">
              <li className="flex items-start space-x-2">
                <span className="text-cyan-400 mt-1">▸</span>
                <span>Progressive Web Apps (PWAs) with offline capabilities</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-cyan-400 mt-1">▸</span>
                <span>React, Vue, and Angular applications</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-cyan-400 mt-1">▸</span>
                <span>Cross platform mobile development</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-cyan-400 mt-1">▸</span>
                <span>E-commerce platforms with custom payment solutions</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-cyan-400 mt-1">▸</span>
                <span>API development and third-party integrations</span>
              </li>
            </ul>
          </div>
          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-white">Technologies</h3>
            <div className="flex flex-wrap gap-2">
              <span className="px-3 py-1 bg-cyan-400/20 border border-cyan-400/30 rounded-full text-sm text-white">React</span>
              <span className="px-3 py-1 bg-cyan-400/20 border border-cyan-400/30 rounded-full text-sm text-white">Vue.js</span>
              <span className="px-3 py-1 bg-cyan-400/20 border border-cyan-400/30 rounded-full text-sm text-white">Angualar</span>
              <span className="px-3 py-1 bg-cyan-400/20 border border-cyan-400/30 rounded-full text-sm text-white">Node.js</span>
              <span className="px-3 py-1 bg-cyan-400/20 border border-cyan-400/30 rounded-full text-sm text-white">Nest.js</span>
              <span className="px-3 py-1 bg-cyan-400/20 border border-cyan-400/30 rounded-full text-sm text-white">Laravel</span>
              <span className="px-3 py-1 bg-cyan-400/20 border border-cyan-400/30 rounded-full text-sm text-white">Django</span>
              <span className="px-3 py-1 bg-cyan-400/20 border border-cyan-400/30 rounded-full text-sm text-white">Next.js</span>
              <span className="px-3 py-1 bg-cyan-400/20 border border-cyan-400/30 rounded-full text-sm text-white">Flutter</span>
            </div>
          </div>
        </div>
      )
    },
    design: {
      title: "Design & Branding",
      content: (
        <div className="space-y-6">
          <p className="text-lg leading-relaxed text-gray-300">
            We create compelling visual identities that resonate with your audience and stand the test of time. From concept to execution, we bring your brand's story to life.
          </p>
          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-white">Design Services</h3>
            <ul className="space-y-2 text-gray-300">
              <li className="flex items-start space-x-2">
                <span className="text-pink-400 mt-1">▸</span>
                <span>Logo design and brand identity systems</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-pink-400 mt-1">▸</span>
                <span>UI/UX design for web and mobile</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-pink-400 mt-1">▸</span>
                <span>Brand guidelines and style guides</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-pink-400 mt-1">▸</span>
                <span>Print design and marketing materials</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-pink-400 mt-1">▸</span>
                <span>Packaging design and product branding</span>
              </li>
            </ul>
          </div>
          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-white">Design Tools</h3>
            <div className="flex flex-wrap gap-2">
              <span className="px-3 py-1 bg-pink-400/20 border border-pink-400/30 rounded-full text-sm text-white">Figma</span>
              <span className="px-3 py-1 bg-pink-400/20 border border-pink-400/30 rounded-full text-sm text-white">Canva</span>
              <span className="px-3 py-1 bg-pink-400/20 border border-pink-400/30 rounded-full text-sm text-white">Adobe Creative Suite</span>
              <span className="px-3 py-1 bg-pink-400/20 border border-pink-400/30 rounded-full text-sm text-white">Sketch</span>
              <span className="px-3 py-1 bg-pink-400/20 border border-pink-400/30 rounded-full text-sm text-white">Illustrator</span>
            </div>
          </div>
        </div>
      )
    },
    motion: {
      title: "Motion & Content",
      content: (
        <div className="space-y-6">
          <p className="text-lg leading-relaxed text-gray-300">
            We transform static ideas into dynamic experiences that capture attention and drive engagement through motion graphics and strategic content creation.
          </p>
          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-white">Motion & Content Services</h3>
            <ul className="space-y-2 text-gray-300">
              <li className="flex items-start space-x-2">
                <span className="text-orange-400 mt-1">▸</span>
                <span>Animated explainer videos and presentations</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-orange-400 mt-1">▸</span>
                <span>Social media content and motion graphics</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-orange-400 mt-1">▸</span>
                <span>3D animations and product visualizations</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-orange-400 mt-1">▸</span>
                <span>Video editing and post-production</span>
              </li>
              <li className="flex items-start space-x-2">
                <span className="text-orange-400 mt-1">▸</span>
                <span>Content strategy and copywriting</span>
              </li>
            </ul>
          </div>
          <div className="space-y-4">
            <h3 className="text-xl font-semibold text-white">Production Tools</h3>
            <div className="flex flex-wrap gap-2">
              <span className="px-3 py-1 bg-orange-400/20 border border-orange-400/30 rounded-full text-sm text-white">After Effects</span>
              <span className="px-3 py-1 bg-orange-400/20 border border-orange-400/30 rounded-full text-sm text-white">Premiere Pro</span>
              <span className="px-3 py-1 bg-orange-400/20 border border-orange-400/30 rounded-full text-sm text-white">Cinema 4D</span>
              <span className="px-3 py-1 bg-orange-400/20 border border-orange-400/30 rounded-full text-sm text-white">Blender</span>
            </div>
          </div>
        </div>
      )
    }
  };

  const service = serviceDetails[serviceId as keyof typeof serviceDetails];
  if (!service) return null;

  return (
    <>
      <h2 className="text-2xl font-bold text-white">{service.title}</h2>
      {service.content}
    </>
  );
};

export default ServiceDetails;
